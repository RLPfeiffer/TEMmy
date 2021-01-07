use duct::cmd;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::time::Duration;
use std::thread;
use std::thread::JoinHandle;

fn run_and_filter_output<F>(command: &str, process_line: F) -> Result<i32, Error> 
    where F: Fn(String) -> () {
    let command = cmd!("cmd.exe", "/C", command);
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
    match run_and_filter_output("@echo Hello, world!", |s| assert_eq!("Hello, world!", s)) {
        Ok(code) => assert_eq!(0, code),
        Err(err) => panic!("error {}", err)
    };
    match run_and_filter_output("@echo Hello, world! 1>&2", |s| assert_eq!("Hello, world! ", s)) {
        Ok(code) => assert_eq!(0, code),
        Err(err) => panic!("error {}", err)
    };
    match run_and_filter_output("exit /b 1", |s| println!("{}", s)) {
        Ok(code) => assert_eq!(1, code),
        Err(err) => panic!("error {}", err)
    };
}

fn run_chain_and_filter_output<F, G>(commands:Vec<&str>, process_line: F, command_on_error: &str, process_line_on_error: G) -> Result<i32, Error>
    where F: Fn(String) -> (),
            G: Fn(String) -> () {
    
    for command in commands {
        match run_and_filter_output(command, &process_line) {
            Ok(0) => continue,
            Ok(error_code) => {
                println!("Error code {} from {}", error_code, command);
                return run_and_filter_output(command_on_error, process_line_on_error);
            },
            Err(err) => {
                println!("Error {} from {}", err, command);
                return run_and_filter_output(command_on_error, process_line_on_error);
            },
        }
    }
    Ok(0)
}

fn run_on_interval_and_filter_output<F, G>(command: &str, process_line: F, seconds: u64, command_on_error: &str, process_line_on_error: G) -> Result<i32, Error>
    where F: Fn(String) -> (),
            G: Fn(String) -> () {
    
    loop {
        match run_and_filter_output(&command, &process_line) {
            Ok(0) => (),
            Ok(error_code) => {
                println!("Error code {} from {}", error_code, command);
                return run_and_filter_output(command_on_error, process_line_on_error);
            },
            Err(err) => {
                println!("Error {} from {}", err, command);
                return run_and_filter_output(command_on_error, process_line_on_error);
            },
        }
        thread::sleep(Duration::from_secs(seconds));
    }
}



fn spawn_tem_message_reader_thread(tem_name: &'static str) -> JoinHandle<()> {
    thread::spawn(move || {
        run_on_interval_and_filter_output(
            format!(r#"type N:\{0}\message.txt && break>N:\{0}\message.txt"#, tem_name).as_str(),
            |output| {
                // TODO do things with the message file lines
                println!("{}", output);
            },
            60,
            format!(r#"rito --slack tem-bot "bob the builder {} thread failed""#, tem_name).as_str(),
            |_output| {});
    })
}

fn main() {
    let t1 = spawn_tem_message_reader_thread("TEM1");
    let t2 = spawn_tem_message_reader_thread("TEM2");

    t1.join();
    t2.join();
}