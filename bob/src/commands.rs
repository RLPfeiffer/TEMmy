use crate::CommandChain;
use CommandBehavior::*;
use crate::lines_from_file;
use crate::ShouldPrint::*;
use crate::rito::*;
use crate::run::*;
use crate::config::*;
use crate::rc3_builds::*;
use crate::core_builds::*;
use crate::robocopy::RobocopyType::*;
use crate::robocopy::*;

use std::collections::HashMap;
pub enum CommandBehavior {
    Immediate(CommandChain),
    Queue(CommandChain),
    NoOp,
}

pub fn command_map() -> HashMap<String, fn(Vec<String>) -> Option<CommandBehavior>> {
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

fn build_chain(section:&str, is_rebuild: bool) -> Option<CommandChain> {
    if section.starts_with("core") {
        core_build_chain(section.to_string(), is_rebuild)
    }
    else {
        rc3_build_chain(section.to_string(), is_rebuild)
    }
}


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
                            commands: vec![robocopy(typ, point_a.clone(), point_b.clone())], 
                        }))
                    },
                    _ => None
                }
            },
            _ => None
        }
}