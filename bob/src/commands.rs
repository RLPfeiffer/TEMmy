use crate::CommandChain;
use CommandBehavior::*;
use crate::lines_from_file;
use crate::ShouldPrint::*;
use crate::rito::*;
use crate::run::*;
use crate::config::*;
use crate::robocopy::RobocopyType::*;
use crate::robocopy::*;
use crate::volume::Volume;

use std::collections::HashMap;
pub enum CommandBehavior {
    Immediate(CommandChain),
    Queue(CommandChain),
    NoOp,
}

pub type CommandMap = HashMap<String, fn(Vec<String>) -> Option<CommandBehavior>>;

pub fn command_map() -> CommandMap {
    let mut commands:CommandMap = HashMap::new();
    
    commands.insert("Copied".to_string(), |args| build_command(true, args));
    commands.insert("Build".to_string(), |args| build_command(false, args));
    commands.insert("Rebuild".to_string(), |args| build_command(false, args));

    // Merge automatically-built sections with a full volume
    commands.insert("Merge".to_string(), merge_command);

    commands.insert("FixMosaic".to_string(), |args| fixmosaic_command(true, args));
    commands.insert("FixMosaicStage".to_string(), |args| fixmosaic_command(false, args));

    /*
    // Deploy a core capture volume
    commands.insert("Deploy".to_string(), |args| {
        match args.as_slice() {
            [volume] => Some(Queue(core_deploy_chain(volume.clone()))),
            _ => None
        }
    });
    
    */
    // Send snapshots to #tem-bot as images
    commands.insert("Snapshot".to_string(), |args| {
        // The snapshot command accepts a filename that can include spaces, so the args vec actually needs to be rejoined:
        let snapshot_name = args.join(" ");
        let config = config_from_yaml();
        let snapshot_path = format!(r#"{}\{}"#, config.dropbox_dir, snapshot_name);
        Some(
            Immediate(
                CommandChain {
                    folders_to_lock: vec![],
                    commands: vec![
                        rito_image(snapshot_path)
                    ],
                    label: "send snapshot to slack".to_string()
                }))
    });
    // Started: is just intended to send a message
    // queue commands from a text file and save their outputs:
    commands.insert("Queue".to_string(), |args| {
        match args.as_slice() {
            [queue_file] => {
                println!("called as queue");
                match lines_from_file(queue_file) {
                    Ok(queue) => {
                        let queue: Vec<Vec<String>> = queue.iter().map(|line| line.split("~").map(|token| token.trim().to_string()).collect()).collect();
                        Some(Queue(CommandChain {
                            commands: queue, 
                            folders_to_lock: vec![],
                            label: format!("bob queue file {}", queue_file)
                        }))
                    },
                    Err(err) => {
                        run_warn(rito(format!("error reading lines from queue file {}: {:?}", queue_file, err)), Print);
                        None
                    }
                }
                // TODO tokenize queue files by passing the lines through a filter that just prints each arg on a line
                // TODO The file has to tokenize command arguments like~this~"even though it's weird"
            },
            _ => None
        }
    });
    // Run robocopy to copy a folder
    commands.insert("Copy".to_string(), |args| {
        robocopy_command(Copy, args)
    });
    // Run robocopy to move a folder
    commands.insert("Move".to_string(), |args| {
        robocopy_command(Move, args)
    });
    // Add a raw shell command to the queue (i.e. RC3Align)
    commands.insert("Raw".to_string(), |args| {
        match args.as_slice() {
            [command_string] => {
                let command:Vec<String> = command_string.split("~").map(|arg| arg.trim().to_string()).collect();
                Some(Queue(CommandChain {
                    folders_to_lock: vec![],
                    commands: vec![
                        command,
                    ],
                    label: format!("raw command '{}'", command_string),
                }))
            },
            _ => None
        }
    });
    commands
}

fn volume_for<'a>(name:String, config:&'a Config) -> Option<&'a Volume> {
    for volume_config in &config.volumes {
        if volume_config.name == name.clone() {
            return Some(&volume_config);
        }
    }
    None
}

fn build_command(is_automatic: bool, args:Vec<String>) -> Option<CommandBehavior> {
    // Copied: can have multiple plaintext words after the section name/number
    if args.len() >= 2 {
        let volume = &args[0];
        let section = &args[1];
        
        let config = config_from_yaml();
        if config.automatic_builds || !is_automatic {
            if let Some(volume_config) = volume_for(volume.clone(), &config) {
                return if let Ok(chain) = volume_config.build_chain(section.to_string()) {
                    Some(Queue(chain))
                } else {
                    None
                }
            }
            return None
        } else {
            return Some(NoOp)
        }
    } else {
        None
    }
}

fn merge_command(args:Vec<String>) -> Option<CommandBehavior> {
    // Merge: <Volume name> <section>
    if args.len() == 2 {
        let volume = &args[0];
        let section = &args[1];
        
        let config = config_from_yaml();
        if let Some(volume_config) = volume_for(volume.clone(), &config) {
            return if let Ok(chain) = volume_config.merge_chain(section.to_string()) {
                Some(Queue(chain))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn fixmosaic_command(stage:bool, args:Vec<String>) -> Option<CommandBehavior> {
    // <FixMosaic or FixMosaicStage> : <Volume name> <section>
    if args.len() == 2 {
        let volume = &args[0];
        let section = &args[1];
        
        let config = config_from_yaml();
        if let Some(volume_config) = volume_for(volume.clone(), &config) {
            return if let Ok(chain) = volume_config.fixmosaic_chain(section.to_string(), stage) {
                Some(Queue(chain))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn robocopy_command(typ:RobocopyType, args:Vec<String>) -> Option<CommandBehavior> {
    match args.as_slice() {
            [exp] => {
                // TODO tokenize folders by passing the lines through a filter that just prints each arg on a line
                // TODO The file has to tokenize source and dest like->this
                let folders: Vec<String> = exp.split("->").map(|token| token.trim().to_string()).collect();
                match folders.as_slice() {
                    [point_a, point_b] => {
                        Some(Queue(CommandChain {
                            label: format!("bob robocopy {:?} {} -> {}", typ, point_a, point_b),
                            folders_to_lock: vec![point_a.clone(), point_b.clone()], 
                            commands: vec![robocopy(typ, point_a.clone(), point_b.clone())], 
                        }))
                    },
                    _ => None
                }
            },
            _ => None
        }
}