use duct::cmd;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Error;
use std::time::Duration;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::SystemTime;
use std::fs;
use humantime::format_rfc3339;
extern crate yaml_rust;
use yaml_rust::YamlLoader;

struct Config {
    dropbox_dir: String,
    dropbox_link_dir: String,
    build_target: String,
    raw_data_dir: String,
    notification_dir: String,
    core_deployment_dir: String,
}

fn config_from_yaml() -> Config {
    let yaml_str = fs::read_to_string("bob-config.yaml").expect("bob requires a bob-config.yaml file");
    let docs = YamlLoader::load_from_str(&yaml_str).unwrap();
    let yaml = &docs[0];
    Config {
        dropbox_dir: yaml["dropbox_dir"].as_str().unwrap().to_string(),
        dropbox_link_dir: yaml["dropbox_link_dir"].as_str().unwrap().to_string(),
        build_target: yaml["build_target"].as_str().unwrap().to_string(),
        raw_data_dir: yaml["raw_data_dir"].as_str().unwrap().to_string(),
        notification_dir: yaml["notification_dir"].as_str().unwrap().to_string(),
        core_deployment_dir: yaml["core_deployment_dir"].as_str().unwrap().to_string(),
    }
    // TODO have a list of volumes in the yaml file and let them define
    // import/build/merge/align script chains
}

fn run_and_filter_output<F>(command: Vec<String>, mut process_line: F) -> Result<i32, Error> 
    where F: FnMut(String) -> () {
    let mut args = vec![String::from("/C")];
    for arg in command {
        args.push(arg);
    }
    let command = cmd("cmd.exe", args);
    let reader = command.stderr_to_stdout().reader()?;
    let lines = BufReader::new(&reader).lines();
    for line in lines {
        match line {
            Ok(line) => process_line(line),
            Err(err) => return {
                let err_str = format!("{}", err);
                if err_str.contains("exited with code") {
                    Ok(err_str.split(" ")
                                .collect::<Vec<&str>>()
                                .pop().unwrap()
                                .parse::<i32>().unwrap())
                } else {
                    Err(err)
                }
            },
        }
    }
    Ok(0)
}

#[test]
fn test_run_and_filter_output() {
    match run_and_filter_output(vec!["@echo Hello, world!".to_string()], |s| assert_eq!("Hello, world!", s)) {
        Ok(code) => assert_eq!(0, code),
        Err(err) => panic!("error {}", err)
    };
    match run_and_filter_output(vec!["@echo Hello, world! 1>&2".to_string()], |s| assert_eq!("Hello, world! ", s)) {
        Ok(code) => assert_eq!(0, code),
        Err(err) => panic!("error {}", err)
    };
    match run_and_filter_output(vec!["exit /b 1".to_string()], |s| println!("{}", s)) {
        Ok(code) => assert_eq!(1, code),
        Err(err) => panic!("error {}", err)
    };
    match run_and_filter_output(vec!["@echo".to_string(), "Hey".to_string(), ">".to_string(), "file.txt".to_string()], |_| {}) {
        Ok(code) => assert_eq!(0, code),
        Err(err) => panic!("error {}", err)
    };
}

fn run_and_print_output(command: Vec<String>) -> Result<i32, Error> {
    run_and_filter_output(command, |output| {
        println!("{}", output);
    })
}

// TODO allow parsing .cmd files into this (will need to arguments tokenize properly) (ignoring REM and other things)
struct CommandChain {
    commands: Vec<Vec<String>>,
    command_on_error: Vec<String>,
}

fn run_chain_and_save_output(chain: CommandChain) -> Result<i32, Error> {
    let commands = chain.commands;
    let command_on_error = chain.command_on_error;
    for command in commands {
        let timestamp = make_timestamp();
        // make a folder for bob output files
        if fs::create_dir("bob-output").is_ok() {
            println!("created bob-output directory");
        } 
        let output_file = format!("bob-output/{}.txt", timestamp);
        let file = File::create(output_file).unwrap();
        let mut buffer = BufWriter::new(file);
        buffer.write_all(format!("{:?}\n", command).as_bytes()).unwrap();
        buffer.flush().unwrap();
        println!("{} {:?}", timestamp, command);
        let is_robocopy = command[0] == "robocopy";
        match run_and_filter_output(command.clone(), |line| {
            buffer.write_all(line.as_bytes()).unwrap();
            buffer.write_all(b"\r\n").unwrap();
            buffer.flush().unwrap();
        }) {
            Ok(0) => continue,
            Ok(error_code) if is_robocopy && (error_code == 1 || error_code == 3) => continue, // Robocopy returns 1 on success. yikes
            Ok(error_code) => {
                println!("Error code {} from {:?}", error_code, command);
                return run_and_print_output(command_on_error);
            },
            Err(err) => {
                println!("Error {} from {:?}", err, command);
                return run_and_print_output(command_on_error);
            },
        }
    }
    Ok(0)
}

fn run_on_interval_and_filter_output<F>(command: Vec<String>, process_line: F, seconds: u64, command_on_error: Vec<String>) -> Result<i32, Error>
    where F: Fn(String) -> () {
    
    loop {
        match run_and_filter_output(command.clone(), &process_line) {
            Ok(0) => (),
            Ok(error_code) => {
                println!("Error code {} from {:?}", error_code, command);
                return run_and_print_output(command_on_error);
            },
            Err(err) => {
                println!("Error {} from {:?}", err, command);
                return run_and_print_output(command_on_error);
            },
        }
        thread::sleep(Duration::from_secs(seconds));
    }
}

fn send_rc3_build_chain(section: String, is_rebuild: bool, sender: &Sender<CommandChain>) {
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

        // Move the automatic build's mosaicreport files to DROPBOX and send a link.
        // If the mosaicreport files aren't there, the chain will fail (as it should) because that's
        // a secondary indicator of build failure
        robocopy_move(
            format!(r#"{}\MosaicReport"#, temp_volume_dir.clone()),
            format!(r#"{}\MosaicReports\{}\MosaicReport\"#, config.dropbox_dir, section)),
        vec![
            "move".to_string(),
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

    sender.send(
        CommandChain {
            commands: commands,
            command_on_error: rito(format!("automatic copy and build for {} failed", section))
        }).unwrap();
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
            command_on_error: rito(format!("automatic merge for {} failed", section))
        }).unwrap();
}

fn send_core_build_chain(section: String, _is_rebuild: bool, sender: &Sender<CommandChain>) {
    let config = config_from_yaml();

    let section_dir = format!(r#"{}\TEMXCopy\{}"#, config.dropbox_dir, section);
    let section_parts = section.split("_").collect::<Vec<&str>>();

    match &section_parts[..] {
        ["core", volume, section_number] => {
            let volume_dir = format!(r#"{}\TEMXCopy\{}"#, config.dropbox_dir, volume.clone());
            let mosaic_report_dest = format!(r#"{}\MosaicReports\{}\MosaicReport.html"#, config.dropbox_link_dir, volume.clone());
            let build_target = format!(r#"{}\{}"#, config.build_target, volume.clone());

            // If the volume dir doesn't exist, make it
            fs::create_dir_all(&volume_dir).unwrap();

            sender.send(
                CommandChain {
                    commands: vec![
                        // TODO rebuilds don't need to do this:
                        // Move section into volume dir
                        vec![
                            "move".to_string(),
                            section_dir,
                            format!(r#"{}\{}"#, volume_dir.clone(), section_number.clone()),
                        ],

                        // Run TEMCoreBuildFast
                        vec![
                            "TEMCoreBuildFast".to_string(),
                            build_target.clone(),
                            volume_dir.clone(),
                        ],

                        // Move the automatic build's mosaicreport files to DROPBOX and send a link.
                        // If the mosaicreport files aren't there, the chain will fail (as it should) because that's
                        // a secondary indicator of build failure
                        robocopy_move(
                            format!(r#"{}\MosaicReport"#, build_target.clone()),
                            format!(r#"{}\MosaicReports\{}\MosaicReport\"#, config.dropbox_dir, volume.clone())),
                        vec![
                            "move".to_string(),
                            format!(r#"{}\MosaicReport.html"#, build_target.clone()),
                            mosaic_report_dest.clone(),
                        ],
                        rito(format!("{0} built automatically. Check {1}, and if all sections have been built properly, run `Deploy: {0}` if it looks good", volume, mosaic_report_dest)),
                    ],
                    command_on_error: rito(format!("automatic core build for {0} failed", section))
                }).unwrap();
        },
        _ => {
            run_and_print_output(rito(format!("{0} should be named with pattern core_[volume]_[section] and was not built automatically", section))).unwrap();
        },
    };
    
}

fn send_core_deploy_chain(volume: String, sender: &Sender<CommandChain>) {
    let config = config_from_yaml();

    let volume_dir = format!(r#"{}\{}"#, config.build_target.clone(), volume.clone());
    let deploy_dir = format!(r#"{}\{}"#, config.core_deployment_dir.clone(), volume.clone());

    sender.send(
        CommandChain {
            commands: vec![
                robocopy_move(
                    volume_dir.clone(),
                    format!(r#"{}\"#,deploy_dir.clone())),
                vec![
                    "TEMCoreBuildOptimizeTiles".to_string(),
                    deploy_dir.clone(),
                ],
                vec![
                    "add-volume-path".to_string(),
                    format!(r#"{}\Mosaic.VikingXML"#, deploy_dir.clone()),
                    "bob-output".to_string() // backup dir for Mosaic.VikingXML files
                ],
                rito(format!(r#"{0} might be ready! Check http://storage1.connectomes.utah.edu/{0}/Mosaic.VikingXML in Viking"#, volume))
            ],
            command_on_error: rito(format!("automatic core deployment for {0} failed", volume))
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
        run_on_interval_and_filter_output(
            vec![format!(r#"type {0}\{1}\message.txt && break>{0}\{1}\message.txt"#, config.notification_dir, tem_name)],
            |output| {
                sender.send(format!("{}: {}", tem_name, output)).unwrap();
            }, 
            60,
            rito(format!("bob the builder {} thread failed", tem_name))).unwrap();
        }
    )
}

fn rito(message: String) -> Vec<String> {
    vec!["rito".to_string(), "--slack".to_string(), "tem-bot".to_string(), message]
}

fn rito_image(path: String) -> Vec<String> {
    vec!["rito".to_string(), "--slack_image".to_string(), "tem-bot".to_string(), path]
}

fn robocopy_move<'a>(source: String, dest: String) -> Vec<String> {
    vec![
        "robocopy".to_string(),
        source,
        dest,
        "/MT:32".to_string(),
        "/LOG:RC3Robocopy.log".to_string(),
        "/MOVE".to_string(),
        "/nfl".to_string(),
        "/nc".to_string(),
        "/ns".to_string(),
        "/np".to_string(),
        "/E".to_string(),
        "/TEE".to_string(),
        "/R:3".to_string(),
        "/W:1".to_string(),
        "/REG".to_string(),
        "/DCOPY:DAT".to_string(),
        "/XO".to_string(),
    ]
}

use rustyline::error::ReadlineError;
use rustyline::Editor;

// TODO cli thread should allow raw commands to be run
// TODO cli thread could allow serialization/suspension of chains to restart bob????
// TODO set up commandchain reading so that .cmd files are already valid (ignore rem, tokenize, etc)
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
        }
        rl.save_history("history.txt").unwrap();
    })
    // TODO raw command


}

fn spawn_worker_thread(receiver: Receiver<CommandChain>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let next_chain = receiver.recv().unwrap();
            run_chain_and_save_output(next_chain).unwrap();
        }
    })
}

fn spawn_command_thread(receiver: Receiver<String>, sender: Sender<CommandChain>) -> JoinHandle<()> {
    let config = config_from_yaml();
    thread::spawn(move || {
        loop {
            let next_command = receiver.recv().unwrap();
            println!("saving the Message output: {}", next_command);
            let mut tokens = next_command.split(": ");
            let tem_name = tokens.next().unwrap();
            run_and_print_output(vec![format!(r#"@echo {} >> {}\{}\processedMessage.txt"#, config.notification_dir, next_command, tem_name)]).unwrap();
            match tokens.next() {
                Some("Copied") => {
                    let section = tokens.next().unwrap().split(" ").next().unwrap();
                    // handle core builds with TEMCoreBuildFast
                    if section.starts_with("core") {
                        send_core_build_chain(section.to_string(), false, &sender);
                    }
                    // handle RC3 builds by importing and building, then copying to rawdata
                    else {
                        println!("{}", section);
                        // copy to rawdata, automatically build to its own section
                        // (but do this in another thread, so notifications still pipe to Slack for other messages)
                        send_rc3_build_chain(section.to_string(), false, &sender);
                    }
                },
                Some("Rebuild") => {
                    let section = tokens.next().unwrap().split(" ").next().unwrap();
                    // handle core builds with TEMCoreBuildFast
                    if section.starts_with("core") {
                        send_core_build_chain(section.to_string(), true, &sender);
                    }
                    // handle RC3 rebuilds by building FROM rawdata
                    else {
                        println!("rebuilding {}", section);
                        // build FROM rawdata, and don't need to copy to it afterwards.
                        send_rc3_build_chain(section.to_string(), true, &sender);
                    }

                },
                // Send snapshots to #tem-bot as images
                Some("Snapshot") => {
                    let snapshot_name = tokens.next().unwrap();
                    let snapshot_path = format!(r#"{}\{}"#, config.dropbox_dir, snapshot_name);
                    run_chain_and_save_output(
                        CommandChain {
                            commands: vec![
                                rito_image(snapshot_path),
                            ],

                            command_on_error: rito(format!("snapshot -> slack failed for {}", snapshot_name))
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
                        command_on_error: rito(format!("bob queue {} failed", queue_file))
                    }).unwrap();
                },
                // Add a raw shell command to the queue (i.e. RC3Align)
                Some("Raw") => {
                    let command_string = tokens.next().unwrap();
                    let command:Vec<String> = command_string.split("~").map(|arg| arg.trim().to_string()).collect();
                    sender.send(CommandChain {
                        commands: vec![command],
                        command_on_error: rito(format!("bob raw command '{}' failed", command_string))
                    }).unwrap();
                },
                // Deploy a core capture volume
                Some("Deploy") => {
                    let volume = tokens.next().unwrap();
                    send_core_deploy_chain(volume.to_string(), &sender);
                },
                // This case will be used even if there's no colon in the message
                Some(other_label) => {
                    println!("{}", other_label);
                    run_and_print_output(rito(next_command)).unwrap();
                },
                // This case should never be used
                None => ()
            }
        }
    })
}

fn make_timestamp() -> String {
    let sys_time = SystemTime::now();
    format!("{}", format_rfc3339(sys_time)).replace(":", "-").replace(".", "-")
}

#[test]
fn test_make_timestamp() {
    // The timestamp needs to contain hour/min/sec to disambiguate from others
    match make_timestamp() {
        stamp if stamp.chars().count() >= 20 => println!("{}", stamp),
        _ => panic!("timestamp too short")
    };
}
 
fn main() {
    // Create a channel for all Bob commands to be sent safely to a command processor thread:
    let (command_sender, command_receiver) = channel();

    // Create a channel for all Bob jobs to be sent safely to a single worker thread as CommandChains:
    let (chain_sender, chain_receiver) = channel();

    spawn_command_thread(command_receiver, chain_sender);
    spawn_worker_thread(chain_receiver);    

    // Two threads simply monitor the notification text files from the TEMs,
    // and will send lines from them to the command processor thread
    spawn_tem_message_reader_thread("TEM1", command_sender.clone());
    spawn_tem_message_reader_thread("TEM2", command_sender.clone());

    // The CLI thread listens for manually entered CommandChains via queues or raw commands
    spawn_cli_thread(command_sender.clone()).join().unwrap();
}