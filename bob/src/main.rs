use std::io::prelude::*;
use std::io::BufReader;
use std::process::{Command, Stdio};

fn run_and_filter_output<F>(command: &str, process_line: F) -> Result<i32, std::io::Error> 
    where F: Fn(String) -> () {
    match Command::new("cmd.exe")
                    .arg("/C")
                    .arg(command)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn() {
        Ok(mut process) => {
            let output = process.stdout.take().unwrap();
            let mut reader = BufReader::new(output);
            loop {
                let mut s = String::new();
                match reader.read_line(&mut s) {
                    Ok(0) => return Ok(match process.try_wait() {
                        Ok(Some(status)) => status.code().unwrap(),
                        Ok(None) => process.wait().expect("").code().unwrap(),
                        Err(err) => return Err(err) 
                    }),
                    Ok(_) => process_line(s),
                    Err(err) => return Err(err)
                }
            }
        },
        Err(err) => Err(err),
    }
}

fn main() {
    match run_and_filter_output("@echo Hello, world!", |s| println!("{}", s)) {
        Ok(code) => println!("finished with {}", code),
        Err(err) => println!("error {}", err)
    };
}

