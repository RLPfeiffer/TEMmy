use whoami::{realname,devicename};
use regex::Regex;
use std::fs;
use std::fs::File;
use std::thread;
use crate::config::config_from_yaml;
use duct::cmd;
use std::io::BufReader;
use std::io::BufRead;
use std::io::prelude::*;
use std::time::Duration;
use std::io::BufWriter;
use crate::rito::*;
use std::time::SystemTime;
use humantime::format_rfc3339;
use crate::errors::*;
use crate::run::ShouldPrint::*;

pub type Command = Vec<String>;

pub enum ShouldPrint {
    Print,
    Silent,
}

// Run, save output, optionally print output, report errors via slack and terminal 
pub fn run(command: Vec<String>, print_output:ShouldPrint) -> BobResult<()> {
    let timestamp = make_timestamp();
    // make a folder for bob output files
    if fs::create_dir("bob-output").is_ok() {
        println!("created bob-output directory");
    } 
    let output_file = format!("bob-output/{}.txt", timestamp);
    let file = File::create(output_file.clone())?;
    let mut buffer = BufWriter::new(file);
    buffer.write_all(format!("{:?}\n", command).as_bytes())?;
    buffer.flush().unwrap();
    println!("{} {:?}", output_file.clone(), command);
    
    let mut process_line = |line:String| {
        buffer.write_all(line.as_bytes())?;
        buffer.write_all(b"\r\n")?;
        buffer.flush()?;
        match print_output {
            Print => println!("{}", line),
            Silent => ()
        };
        Ok(())
    };

    let result = run_and_filter_output(command.clone(), &mut process_line);
    if result.is_err() {
        if run(rito_file(output_file.clone()), Print).is_err() {
            panic!("Error sending failed command log from {:?}", command);
        }
    }
    result
}

pub fn run_warn(command: Vec<String>, print_output:ShouldPrint) -> () {
    if let Err(err) = run(command.clone(), print_output) {
        println!("Error {} from {:?}", err, command);
    }
}

fn run_and_filter_output<F>(command: Vec<String>, mut process_line: F) -> BobResult<()> 
    where F: FnMut(String) -> BobResult<()> {

    let config = config_from_yaml();

    // Only compile regexes once per command
    let junk_output_regexes: Vec<Regex> = config.junk_outputs.iter().map(|pattern| Regex::new(&pattern).unwrap()).collect();
    let fatal_error_regexes: Vec<Regex> = config.fatal_errors.iter().map(|pattern| Regex::new(&pattern).unwrap()).collect();

    let mut args = vec![String::from("/C")];
    for arg in &command {
        args.push(arg.clone());
    }
    let command_runner = cmd("cmd.exe", args);
    let reader = command_runner.stderr_to_stdout().reader()?;
    let lines = BufReader::new(&reader).lines();
    let mut consecutive_junk_lines = 0;
    let mut last_junk_pattern = "".to_string();
    let is_robocopy = command[0] == "robocopy";

    for line in lines {
        match line {
            Ok(line) => {
                // check if the line matches a set of known junk output patterns, i.e. Jobs Queued: [n]
                let mut is_junk = false;
                for junk_output_regex in &junk_output_regexes {
                    if junk_output_regex.is_match(&line) {
                        is_junk = true;
                        last_junk_pattern = junk_output_regex.to_string();
                        break;
                    }
                }
                if is_junk {
                    consecutive_junk_lines += 1;
                    continue;
                } else if consecutive_junk_lines > 0 {
                    process_line(format!("[{} lines of junk output matching pattern  '{}']", consecutive_junk_lines, last_junk_pattern))?;
                    consecutive_junk_lines = 0;
                }

                // check if the line matches a set of known error patterns, i.e. 64-thread python error
                for fatal_error_regex in &fatal_error_regexes {
                    if fatal_error_regex.is_match(&line) {
                        reader.kill().unwrap();
                        return report_error(BobError::FatalRegex(line), command);
                    }
                }
 
                process_line(line)?;
            },
            Err(err) => return {
                let err_str = format!("{}", err);
                if err_str.contains("exited with code") {
                    let exit_code = err_str.split(" ")
                                .collect::<Vec<&str>>()
                                .pop().unwrap()
                                .parse::<i32>()?;

                    // Robocopy returns 1 and 3 on some successes. yikes
                    if is_robocopy && (exit_code == 1 || exit_code == 3) {
                        Ok(())
                    } else {
                        report_error(BobError::BadExitCode(exit_code), command)
                    }
                } else {
                    report_error(BobError::IOError(err), command)
                }
            },
        }
    }
    Ok(())
}

fn report_error(err: BobError, command: Vec<String>) -> BobResult<()> {
    if run(rito(format!("Error {} from {:?}", err, command)), Print).is_err() {
        panic!("Error reporting error {} from {:?}", err, command);
    }
    Err(err)
}

pub struct CommandChain {
    pub label: String,
    pub commands: Vec<Command>,
}

pub fn run_chain_and_save_output(chain: CommandChain) -> BobResult<()> {
    let commands = chain.commands;
    let label_with_info = format!("{} on {} via {}", chain.label, devicename(), realname());
    run(rito(format!("Starting command chain: {}", label_with_info)), Print)?;
    for command in commands {
        if let Err(err) = run(command.clone(), Silent) {
            run_warn(rito(format!("Command chain failed: {}", label_with_info)), Print);
            return Err(err);
        }
    }
    run(rito(format!("Command chain finished: {}", label_with_info)), Print)
}

pub fn run_on_interval_and_filter_output<F>(command: Vec<String>, process_line: F, seconds: u64) -> Result<(), BobError>
    where F: Fn(String) -> BobResult<()> {
    
    loop {
        run_and_filter_output(command.clone(), &process_line)?;
        thread::sleep(Duration::from_secs(seconds));
    }
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
 