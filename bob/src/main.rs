use std::env;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::convert::TryInto;
use threadpool::ThreadPool;
use std::io::prelude::*;
use std::io::BufReader;
use duct::cmd;

mod config;
use config::*;
mod run;
use run::*;
use run::ShouldPrint::*;
mod rito;
use rito::*;
mod robocopy;
mod errors;
use errors::*;
mod core_builds;
mod rc3_builds;
mod commands;
use commands::*;
use commands::CommandBehavior::*;

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
                let label = next_chain.label.clone();
                if run_chain_and_save_output(next_chain).is_err() {
                    println!("error from {} -- should have been reported via slack also", label);
                };
            });
        }
    })
}

fn command_thread_step(commands: &CommandMap, receiver: &Receiver<String>, sender: &Sender<CommandChain>) -> BobResult<()> {
    let next_command_full = receiver.recv()?;
    println!("saving the Message output: {}", next_command_full);
    let mut command_parts = next_command_full.split(": ");
    let tem_name = command_parts.next().ok_or(BobError::CommandNoneError("TEM Name", next_command_full.clone()))?;
    let command_name = command_parts.next().ok_or(BobError::CommandNoneError("Command name", next_command_full.clone()))?;
    println!("{}", command_name);
    let command_args = command_parts.next().ok_or(BobError::CommandNoneError("Command args", next_command_full.clone()))?.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
    let config = config_from_yaml();
    run_warn(vec![format!(r#"@echo {} >> {}\{}\processedMessage.txt"#, next_command_full, config.notification_dir, tem_name)], Print);

    if let Some(command_behavior) = commands.get(command_name) {
        match command_behavior(command_args) {
            // TODO won't be matching Some, will be matchking Ok()
            Some(Immediate(chain)) => {
                run_chain_and_save_output(chain)?;
            },
            Some(Queue(chain)) => {
                sender.send(chain)?;
            },
            Some(NoOp) => {},
            None => { run_warn(rito(format!("bad bob command (command_behavior returned None): {}", next_command_full)), Print); }
        };
    } else {
        run_warn(rito(format!("bad bob command: {}", next_command_full)), Print);
    }
    Ok(())
}

fn spawn_command_thread(receiver: Receiver<String>, sender: Sender<CommandChain>) -> JoinHandle<()> {
 
    let commands = command_map();

    thread::spawn(move || {
        loop {
            if let Err(err) = command_thread_step(&commands, &receiver, &sender) {
                run_warn(rito(format!("bob command thread error: {}", err)), Print);
            }
        }
    })
}
fn main() {
    let args:Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, arg] if arg.as_str() == "run_unsafe" => {
            unsafe_main();
        },
        [ref bob_path] => {
            loop {
                let unsafe_result = cmd!(bob_path, "run_unsafe").run();
                if let Ok(output) = unsafe_result {
                    let _ = run_warn(rito(format!("bob the builder crashed: {}. Restarting", String::from_utf8(output.stderr).expect("utf-8 output"))), Silent);
                }
            }
        }
        _ => {
            println!("bad args for bob");
        }
    }
}

fn unsafe_main() {
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