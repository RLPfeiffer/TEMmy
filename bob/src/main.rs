use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::convert::TryInto;
use threadpool::ThreadPool;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

mod config;
use config::*;

mod run;
use run::*;
use run::ShouldPrint::*;

mod rito;
use rito::*;

mod robocopy;
use robocopy::*;

mod errors;
use errors::*;

mod core_builds;
use core_builds::*;

mod rc3_builds;
use rc3_builds::*;

// Source: https://stackoverflow.com/a/35820003
use std::{
    fs::File,
    path::Path,
};
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn spawn_tem_message_reader_thread(tem_name: &'static str, sender: Sender<String>) -> JoinHandle<()> {
    let config = config_from_yaml();
    thread::spawn(move || {
        if run_on_interval_and_filter_output(
            vec![format!(r#"type {0}\{1}\message.txt && break>{0}\{1}\message.txt"#, config.notification_dir, tem_name)],
            |output| {
                sender.send(format!("{}: {}", tem_name, output))?;
                Ok(())
            }, 
            60).is_err() {
                run_warn(rito(format!("bob the builder {} thread failed", tem_name)), Silent);
            }
            
        }
    )
}

use rustyline::error::ReadlineError;
use rustyline::Editor;

// TODO cli thread could allow serialization/suspension of chains to restart bob????
fn spawn_cli_thread(sender: Sender<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut rl = Editor::<()>::new();
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    // Pretend it's from a scope called CLI so the command gets saved to processed messages:
                    sender.send(format!("CLI: {}", line)).unwrap();
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break
                },
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
            rl.save_history("history.txt").unwrap();
        }
    })

}

fn spawn_worker_threadpool(receiver: Receiver<CommandChain>) -> JoinHandle<()> {
    let config = config_from_yaml();
    let pool = ThreadPool::new(config.worker_threads.try_into().unwrap());
    thread::spawn(move || {
        loop {
            let next_chain = receiver.recv().unwrap();
            pool.execute(move || {
                run_chain_and_save_output(next_chain).unwrap();
            });
        }
    })
}

fn build_chain(section:&str, is_rebuild: bool) -> Option<CommandChain> {
    if section.starts_with("core") {
        core_build_chain(section.to_string(), is_rebuild)
    }
    else {
        rc3_build_chain(section.to_string(), is_rebuild)
    }
}

enum CommandBehavior {
    Immediate(CommandChain),
    Queue(CommandChain),
    NoOp,
}
use crate::CommandBehavior::*;

fn build_command(is_automatic: bool, is_rebuild: bool, args:Vec<String>) -> Option<CommandBehavior> {
    match args.as_slice() {
        [capture_dir] => {
            let config = config_from_yaml();

            if config.automatic_builds || !is_automatic {
                if let Some(chain) = build_chain(capture_dir, is_rebuild) {
                    Some(Queue(chain))
                } else {
                    None
                }
            } else {
                Some(NoOp)
            }
        },
        _ => None
    }
}

fn spawn_command_thread(receiver: Receiver<String>, sender: Sender<CommandChain>) -> JoinHandle<()> {
 
    // TODO make commands return result<CommandBehavior>
    let mut commands:HashMap<String, fn(Vec<String>) -> Option<CommandBehavior>> = HashMap::new();
    commands.insert("Copied".to_string(), |args| build_command(true, false, args));
    commands.insert("Build".to_string(), |args| build_command(false, false, args));
    commands.insert("Rebuild".to_string(), |args| build_command(false, true, args));
    commands.insert("CoreFixMosaicStage".to_string(), |args| {
        if args.len() < 2 {
            None
        } else {
            let volume = args[0].clone();
            let sections = &args[1..];
            Some(Queue(core_fixmosaic_stage(volume, sections.to_vec())))
        }
    });
    // Deploy a core capture volume
    commands.insert("Deploy".to_string(), |args| {
        match args.as_slice() {
            [volume] => Some(Queue(core_deploy_chain(volume.clone()))),
            _ => None
        }
    });
    // Merge automatically-built RC3 sections with the full volume
    // TODO not all merges will be RC3 forever
    commands.insert("Merge".to_string(), |args| {
        match args.as_slice() {
            [section] => Some(Queue(rc3_merge_chain(section.clone()))),
            _ => None
        }
    });
    // Send snapshots to #tem-bot as images
    commands.insert("Snapshot".to_string(), |args| {
        match args.as_slice() {
            [snapshot_name] => {
                let config = config_from_yaml();
                let snapshot_path = format!(r#"{}\{}"#, config.dropbox_dir, snapshot_name);
                run_warn(rito_image(snapshot_path), Print);
                Some(NoOp)
            },
            _ => None
        }
    });
    // queue commands from a text file and save their outputs:
    commands.insert("Queue".to_string(), |args| {
        match args.as_slice() {
            [queue_file] => {
                println!("called as queue");
                let queue = lines_from_file(queue_file);
                // TODO tokenize queue files by passing the lines through a filter that just prints each arg on a line
                // TODO The file has to tokenize command arguments like~this~"even though it's weird"
                let queue: Vec<Vec<String>> = queue.iter().map(|line| line.split("~").map(|token| token.trim().to_string()).collect()).collect();
                Some(Queue(CommandChain {
                    commands: queue, 
                    label: format!("bob queue file {}", queue_file)
                }))
            },
            _ => None
        }
    });

    // Add a raw shell command to the queue (i.e. RC3Align)
    commands.insert("Raw".to_string(), |args| {
        match args.as_slice() {
            [command_string] => {
                let command:Vec<String> = command_string.split("~").map(|arg| arg.trim().to_string()).collect();
                Some(Queue(CommandChain {
                    commands: vec![
                        command,
                    ],
                    label: format!("raw command '{}'", command_string),
                }))
            },
            _ => None
        }
    });

    thread::spawn(move || {
        loop {
            let next_command_full = receiver.recv().unwrap();
            println!("saving the Message output: {}", next_command_full);
            let mut command_parts = next_command_full.split(": ");
            let tem_name = command_parts.next().unwrap();
            let command_name = command_parts.next().unwrap();
            let command_args = command_parts.next().unwrap().split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
            let config = config_from_yaml();
            run_warn(vec![format!(r#"@echo {} >> {}\{}\processedMessage.txt"#, config.notification_dir, next_command_full, tem_name)], Print);

            if let Some(command_behavior) = commands.get(command_name) {
                match command_behavior(command_args) {
                    // TODO won't be matching Some, will be matchking Ok()
                    Some(Immediate(chain)) => {
                        run_chain_and_save_output(chain).unwrap();
                    },
                    Some(Queue(chain)) => {
                        sender.send(chain).unwrap();
                    },
                    Some(NoOp) => {},
                    None => { run_warn(rito(format!("bad bob command: {}", next_command_full)), Print); }
                };
            } else {
                run_warn(rito(format!("bad bob command: {}", next_command_full)), Print);
            }
       }
    })
}

fn main() {
    let config = config_from_yaml();

    // Create a channel for all Bob commands to be sent safely to a command processor thread:
    let (command_sender, command_receiver) = channel();

    // Create a channel for all Bob jobs to be sent safely to a single worker thread as CommandChains:
    let (chain_sender, chain_receiver) = channel();

    spawn_command_thread(command_receiver, chain_sender);
    spawn_worker_threadpool(chain_receiver);    

    if config.process_tem_output {
        // Two threads simply monitor the notification text files from the TEMs,
        // and will send lines from them to the command processor thread
        spawn_tem_message_reader_thread("TEM1", command_sender.clone());
        spawn_tem_message_reader_thread("TEM2", command_sender.clone());
    }

    // The CLI thread listens for manually entered CommandChains via queues or raw commands
    spawn_cli_thread(command_sender.clone()).join().unwrap();
}