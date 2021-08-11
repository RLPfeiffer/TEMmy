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

fn rc3_build_chain(section: String, is_rebuild: bool) -> Option<CommandChain> {
    let config = config_from_yaml();
    let temp_volume_dir = format!(r#"{}\RC3{}"#, config.build_target, section);
    let mosaic_report_dest = format!(r#"{}\MosaicReports\{}\MosaicReport.html"#, config.dropbox_link_dir, section);
    let source = if is_rebuild {
        format!(r#"{}\RC3\{}"#, config.raw_data_dir, section)
    } else {
        format!(r#"{}\TEMXCopy\{}"#, config.dropbox_dir, section)
    };
    let mut commands = vec![
        vec![
            "RC3Import".to_string(),
            temp_volume_dir.clone(),
            source,
        ],
        vec![
            "RC3Build".to_string(),
            temp_volume_dir.clone()
        ],
        // Automatic build finished with code 0. 

        // TODO check that a tileset was generated.
        // TODO sent the mosaicreport overview to slack

        // Copy the automatic build's mosaicreport files to DROPBOX and send a link.
        // If the mosaicreport files aren't there, the chain will fail (as it should) because that's
        // a secondary indicator of build failure
        robocopy_copy(
            format!(r#"{}\MosaicReport"#, temp_volume_dir.clone()),
            format!(r#"{}\MosaicReports\{}\MosaicReport\"#, config.dropbox_dir, section)),
        vec![
            "copy".to_string(),
            format!(r#"{}\MosaicReport.html"#, temp_volume_dir),
            mosaic_report_dest.clone(),
        ],
        rito(format!("{0} built automatically. Check {1} and run `Merge: {0}` if it looks good", section, mosaic_report_dest)),
    ];

    if !is_rebuild {
        commands.push(robocopy_move(
                format!(r#"{}\TEMXCopy\{}"#, config.dropbox_dir, section),
                format!(r#"{}\RC3\{}\"#, config.raw_data_dir, section)));
        commands.push(rito(format!("{} copied to RawData", section)));
    }

    Some(CommandChain {
        commands: commands,
        label: format!("automatic copy and build for RC3 {}", section)
    })
}

// TODO not all merges will be RC3 merges forever
fn send_rc3_merge_chain(section: String, sender: &Sender<CommandChain>) {
    let config = config_from_yaml();

    let temp_volume_dir = format!(r#"{}\RC3{}"#, config.build_target, section);

    sender.send(
        CommandChain {
            commands: vec![
                vec![    
                    "copy-section-links".to_string(),
                    r#"W:\Volumes\RC3\TEM\VolumeData.xml"#.to_string(), // TODO this is RC3 hard-coded
                    format!(r#"{}\TEM\VolumeData.xml"#, temp_volume_dir),
                    "bob-output".to_string()
                ],
                robocopy_move(
                    format!(r#"{}\TEM"#, temp_volume_dir),
                    r#"W:\Volumes\RC3\TEM\"#.to_string()),
                // Delete the temp volume
                vec![
                    "rmdir".to_string(),
                    "/S".to_string(),
                    "/Q".to_string(),
                    temp_volume_dir
                ],
            ],
            label: format!("automatic merge for {} into RC3", section)
        }).unwrap();
}



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

            let command_behavior = commands.get(command_name).unwrap();

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

            /*match tokens.next() {
                // Send snapshots to #tem-bot as images
                Some("Snapshot") => {
                    let snapshot_name = tokens.next().unwrap();
                    let snapshot_path = format!(r#"{}\{}"#, config.dropbox_dir, snapshot_name);
                    run_chain_and_save_output(
                        CommandChain {
                            commands: vec![
                                rito_image(snapshot_path),
                            ],

                            label: format!("snapshot -> slack for {}", snapshot_name)
                        }).unwrap();
                },
                // Merge automatically-built RC3 sections with the full volume
                Some("Merge") => {
                    let section = tokens.next().unwrap();
                    send_rc3_merge_chain(section.to_string(), &sender);
                },
                // When run with the `queue` subcommand, queue commands from a text file and save their outputs:
                Some("Queue") => {
                    println!("called as queue");
                    // TODO allow passing arguments to a queue
                    // TODO convert cmd files to queue.txt files
            
                    let queue_file = tokens.next().unwrap().split(" ").next().unwrap();
                    let queue = lines_from_file(queue_file);
                    // TODO tokenize queue files by passing the lines through a filter that just prints each arg on a line
                    let queue: Vec<Vec<String>> = queue.iter().map(|line| line.split("~").map(|token| token.trim().to_string()).collect()).collect();
                    // TODO The file has to tokenize command arguments like~this~"even though it's weird"
                    sender.send(CommandChain {
                        commands: queue, 
                        label: format!("bob queue file {}", queue_file)
                    }).unwrap();
                },
                // Add a raw shell command to the queue (i.e. RC3Align)
                Some("Raw") => {
                    let command_string = tokens.next().unwrap();
                    let command:Vec<String> = command_string.split("~").map(|arg| arg.trim().to_string()).collect();
                    sender.send(CommandChain {
                        commands: vec![
                            command,
                        ],
                        label: format!("raw command '{}'", command_string),
                    }).unwrap();
                },
                
                // This case will be used even if there's no colon in the message
                Some(other_label) => {
                    println!("{}", other_label);
                    run_and_print_output(rito(next_command)).unwrap();
                },
                // This case should never be used
                None => ()
            }*/
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