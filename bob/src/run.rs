use whoami::{realname,devicename};
use regex::Regex;
use std::io::ErrorKind;
use std::fs;
use std::fs::File;
use std::thread;
use std::io::Error;
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

pub type Command = Vec<String>;

pub fn run_and_filter_output<F>(command: Vec<String>, mut process_line: F) -> Result<i32, Error> 
    where F: FnMut(String) -> () {

    let config = config_from_yaml();
    // Only compile regexes once per command
    let junk_output_regexes: Vec<Regex> = config.junk_outputs.iter().map(|pattern| Regex::new(&pattern).unwrap()).collect();
    let fatal_error_regexes: Vec<Regex> = config.fatal_errors.iter().map(|pattern| Regex::new(&pattern).unwrap()).collect();

    let mut args = vec![String::from("/C")];
    for arg in command {
        args.push(arg);
    }
    let command = cmd("cmd.exe", args);
    let reader = command.stderr_to_stdout().reader()?;
    let lines = BufReader::new(&reader).lines();
    let mut consecutive_junk_lines = 0;
    let mut last_junk_pattern = "".to_string();

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
                    process_line(format!("[{} lines of junk output matching pattern  '{}']", consecutive_junk_lines, last_junk_pattern));
                    consecutive_junk_lines = 0;
                }

                // check if the line matches a set of known error patterns, i.e. 64-thread python error
                for fatal_error_regex in &fatal_error_regexes {
                    if fatal_error_regex.is_match(&line) {
                        let message = format!("Fatal error `{}` from {:?}", line, command);
                        reader.kill().unwrap();
                        return Err(Error::new(ErrorKind::Other, message));
                    }
                }
 
                process_line(line);
            },
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

pub fn run_and_print_output(command: Vec<String>) -> Result<i32, Error> {
    run_and_filter_output(command, |output| {
        println!("{}", output);
    })
}

pub struct CommandChain {
    pub label: String,
    pub commands: Vec<Command>,
}

pub fn run_chain_and_save_output(chain: CommandChain) -> Result<i32, Error> {
    let commands = chain.commands;
    let label_with_info = format!("{} on {} via {}", chain.label, devicename(), realname());
    run_and_print_output(rito(format!("Starting command chain: {}", label_with_info)))?;
    for command in commands {
        let timestamp = make_timestamp();
        // make a folder for bob output files
        if fs::create_dir("bob-output").is_ok() {
            println!("created bob-output directory");
        } 
        let output_file = format!("bob-output/{}.txt", timestamp);
        let file = File::create(output_file.clone()).unwrap();
        let mut buffer = BufWriter::new(file);
        buffer.write_all(format!("{:?}\n", command).as_bytes()).unwrap();
        buffer.flush().unwrap();
        println!("{} {:?}", output_file.clone(), command);
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
                run_and_print_output(rito(format!("Error code {} from {:?}", error_code, command)))?;
                run_and_print_output(rito_file(output_file.clone()))?;
                return run_and_print_output(rito(format!("Command chain failed: {}", label_with_info)));
            },
            Err(err) => {
                println!("Error {} from {:?}", err, command);
                run_and_print_output(rito(format!("Error {} from {:?}", err, command)))?;
                run_and_print_output(rito_file(output_file.clone()))?;
                return run_and_print_output(rito(format!("Command chain failed: {}", label_with_info)));
            },
        }
    }
    run_and_print_output(rito(format!("Command chain finished: {}", label_with_info)))
}

pub fn run_on_interval_and_filter_output<F>(command: Vec<String>, process_line: F, seconds: u64, command_on_error: Vec<String>) -> Result<i32, Error>
    where F: Fn(String) -> () {
    
    loop {
        match run_and_filter_output(command.clone(), &process_line) {
            Ok(0) => (),
            Ok(error_code) => {
                println!("Error code {} from {:?}", error_code, command);
                run_and_print_output(rito(format!("Error code {} from {:?}", error_code, command)))?;
                return run_and_print_output(command_on_error);
            },
            Err(err) => {
                println!("Error {} from {:?}", err, command);
                run_and_print_output(rito(format!("Error {} from {:?}", err, command)))?;
                return run_and_print_output(command_on_error);
            },
        }
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
 