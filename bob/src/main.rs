use std::env;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::convert::TryInto;
use threadpool::ThreadPool;
use std::io::prelude::*;
use std::io::BufReader;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

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
fn lines_from_file(filename: impl AsRef<Path>) -> BobResult<Vec<String>> {
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    let mut lines:Vec<String> = vec![];
    for line in buf.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

fn spawn_tem_message_reader_thread(tem_name: &'static str, sender: Sender<String>) -> JoinHandle<()> {
    let config = config_from_yaml();
    thread::spawn(move || {
        if run_on_interval_and_filter_output(
            vec![
                format!(r#"type {0}\{1}\message.txt && break>{0}\{1}\message.txt"#, config.notification_dir, tem_name)
            ],
            |output| {
                if output.trim().len() > 0 {
                    sender.send(format!("{}: {}", tem_name, output))?;
                }
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

// return true if the CLI needs to shut down
fn cli_thread_step(rl: &mut Editor<()>, sender: &Sender<String>) -> BobResult<bool> {
    let readline = rl.readline(">> ");
    match readline {
        Ok(line) => {
            rl.add_history_entry(line.as_str());
            // Pretend it's from a scope called CLI so the command gets saved to processed messages:
            sender.send(format!("CLI: {}", line))?;
        },
        Err(ReadlineError::Interrupted) => {
            println!("CTRL-C");
            return Ok(true);
        },
        Err(ReadlineError::Eof) => {
            println!("CTRL-D");
            return Ok(true);
        },
        Err(err) => {
            println!("Readline Error: {:?}", err);
            return Ok(false);
        }
    }
    rl.save_history("history.txt")?;
    Ok(false)
}

// TODO cli thread could allow serialization/suspension of chains to restart bob????
fn spawn_cli_thread(sender: Sender<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut rl = Editor::<()>::new();
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }
        loop {
            match cli_thread_step(&mut rl, &sender) {
                Err(err) => run_warn(rito(format!("bob cli thread error: {}", err)), Print),
                Ok(true) => break, // end the program 
                Ok(false) => {},
            };
        }
    })

}

lazy_static!{
    pub static ref BLOCKED_COMMAND_CHAINS: Mutex<Vec<CommandChain>> = Mutex::new(vec![]);
}

fn threadpool_step(receiver: &Receiver<CommandChain>, pool:&ThreadPool) -> BobResult<()> {
    let next_chain = receiver.recv()?;
    pool.execute(move || {
        let label = next_chain.label.clone();
        let res = run_chain_and_save_output(&next_chain);
        if res.is_err() {
            println!("error from {} -- should have been reported via slack also", label);
        };
        if let Ok(false) = res {
            // Store chains that were blocked where the command thread can access them to send them again
            if let Ok(ref mut blocked_chains) = BLOCKED_COMMAND_CHAINS.lock() {
                blocked_chains.push(next_chain);
            } else {
                run_warn(rito(format!("Error saving a blocked command chain")), Print);
            }
        }
    });
    Ok(())
}

fn spawn_worker_threadpool(receiver: Receiver<CommandChain>) -> JoinHandle<()> {
    let config = config_from_yaml();
    let pool_size = match config.worker_threads.try_into() {
        Ok(size) => size,
        Err(err) => {
            run_warn(rito(format!("worker_threads in bob-config.yml failed to convert to integer: {:?}. using 1 thread", err)), Print);
            1
        }
    };
    let pool = ThreadPool::new(pool_size);
    thread::spawn(move || {
        loop {
            if let Err(err) = threadpool_step(&receiver, &pool) {
                run_warn(rito(format!("error in threadpool step: {}", err)), Print);
            }
        }
    })
}

fn command_thread_step(commands: &CommandMap, receiver: &Receiver<String>, sender: &Sender<CommandChain>, last_blocked_check_time:&mut SystemTime) -> BobResult<()> {
    // Check if blocked command chains are free to run yet
    let now = SystemTime::now();
    let dur = now.duration_since(*last_blocked_check_time)?;
    let blocked_check_mins = 15;
    if dur.as_secs() > blocked_check_mins * 60 {
        *last_blocked_check_time = now;
        
        if let Ok(mut blocked_chains) = BLOCKED_COMMAND_CHAINS.lock() {
            while let Some(chain) = blocked_chains.pop() {
                sender.send(chain)?;
            }
        }
    } 

    // Listen for new commands
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
            // TODO won't be matching Some, will be matching Ok()
            Some(Immediate(chain)) => {
                let can_run = run_chain_and_save_output(&chain)?;
                if !can_run {
                    sender.send(chain)?;
                }
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

    let mut last_blocked_check_time = SystemTime::now();

    thread::spawn(move || {
        loop {
            if let Err(err) = command_thread_step(&commands, &receiver, &sender, &mut last_blocked_check_time) {
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
                // TODO would this return Ok() in all crash cases??
                if let Ok(output) = unsafe_result {
                    run_warn(rito(format!("bob the builder crashed: {}. Restarting", match String::from_utf8(output.stderr) {
                        Ok(err_output) => err_output,
                        Err(err) => format!("non-utf8 output error {}", err)
                    })), Silent);
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

    // Two threads simply monitor the notification text files from the TEMs,
    // and will send lines from them to the command processor thread
    if config.process_tem_output {
        spawn_tem_message_reader_thread("TEM1", command_sender.clone());
        spawn_tem_message_reader_thread("TEM2", command_sender.clone());
    }
    // This thread monitors an input file that is managed by the Bob Web UI. It is not technicaly a tem message reader but the behavior is the same
    spawn_tem_message_reader_thread("BobUI", command_sender.clone());

    // The CLI thread listens for manually entered CommandChains via queues or raw commands
    if spawn_cli_thread(command_sender.clone()).join().is_err() {
        panic!("CLI Thread crashed!");
    }
}