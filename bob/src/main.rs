use duct::cmd;
use uuid::Uuid;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Error;
use std::time::Duration;
use std::thread;
use std::env;
use std::thread::JoinHandle;

fn run_and_filter_output<F>(command: &Vec<&str>, mut process_line: F) -> Result<i32, Error> 
    where F: FnMut(String) -> () {
    let mut args = vec!["/C"];
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
    match run_and_filter_output(vec!["@echo Hello, world!"], |s| assert_eq!("Hello, world!", s)) {
        Ok(code) => assert_eq!(0, code),
        Err(err) => panic!("error {}", err)
    };
    match run_and_filter_output(vec!["@echo Hello, world! 1>&2"], |s| assert_eq!("Hello, world! ", s)) {
        Ok(code) => assert_eq!(0, code),
        Err(err) => panic!("error {}", err)
    };
    match run_and_filter_output(vec!["exit /b 1"], |s| println!("{}", s)) {
        Ok(code) => assert_eq!(1, code),
        Err(err) => panic!("error {}", err)
    };
}

fn run_and_print_output(command: Vec<&str>) -> Result<i32, Error> {
    run_and_filter_output(&command, |output| {
        println!("{}", output);
    })
}

/* If greater flexibility is needed than run_chain_and_save_output, this might be worth bringing back
fn run_chain_and_filter_output<F>(commands:Vec<Vec<&str>>, process_line: F, command_on_error: Vec<&str>) -> Result<i32, Error>
    where F: Fn(String) -> () {

    for command in commands {
        match run_and_filter_output(&command, &process_line) {
            Ok(0) => continue,
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
*/

fn run_chain_and_save_output(commands:Vec<Vec<&str>>, command_on_error: Vec<&str>) -> Result<i32, Error> {
    for command in commands {
        let uuid = Uuid::new_v4();
        let output_file = format!("bob-output/{}.txt", uuid);
        let file = File::create(output_file).unwrap();
        let mut buffer = BufWriter::new(file);
        buffer.write_all(format!("{:?}\n", command).as_bytes()).unwrap();
        buffer.flush().unwrap();
        println!("{} {:?}", uuid, command);
        match run_and_filter_output(&command, |line| {
            buffer.write_all(line.as_bytes()).unwrap();
            buffer.write_all(b"\r\n").unwrap();
            buffer.flush().unwrap();
        }) {
            Ok(0) => continue,
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

fn run_on_interval_and_filter_output<F>(command: Vec<&str>, process_line: F, seconds: u64, command_on_error: Vec<&str>) -> Result<i32, Error>
    where F: Fn(String) -> () {
    
    loop {
        match run_and_filter_output(&command, &process_line) {
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

fn spawn_copy_and_build_thread(section: String) -> JoinHandle<()> {
    thread::spawn(move || {
        run_chain_and_save_output(
            vec![
                vec![
                    "xcopy",
                    format!(r#"Y:\Dropbox\TEMXCopy\{0}"#, section).as_str(),
                    format!(r#"Z:\RawData\RC3\{0}\"#, section).as_str(),
                    "/S"],
                rito(format!("{0} copied to RawData", section).as_str()),
                vec![
                    "RC3Import",
                    format!(r#"D:\Volumes\RC3{0}"#, section).as_str(),
                    format!(r#"Y:\Dropbox\TEMXCopy\{0}"#, section).as_str(),
                ],
                vec![
                    "RC3Build",
                    format!(r#"D:\Volumes\RC3{0}"#, section).as_str(),
                ],
                rito(format!("{0} built automatically. Check it and merge it", section).as_str()),
            ],
            rito(format!("automatic copy and build for {0} failed", section).as_str())
        ).unwrap();
    })
}

fn spawn_core_build_thread(section: String) -> JoinHandle<()> {
    thread::spawn(move || {
        run_chain_and_save_output(
            vec![
                vec![
                    "TEMCoreBuildFast",
                    format!(r#"D:\Volumes\{0}"#, section).as_str(),
                    format!(r#"Y:\Dropbox\TEMXCopy\{0}"#, section).as_str()
                ],
                rito(format!("{0} built automatically.", section).as_str()),
            ],
            rito(format!("automatic core build for {0} failed", section).as_str())
        ).unwrap();
    })
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

fn spawn_tem_message_reader_thread(tem_name: &'static str) -> JoinHandle<()> {
    thread::spawn(move || {
        run_on_interval_and_filter_output(
            vec![format!(r#"type N:\{0}\message.txt && break>N:\{0}\message.txt"#, tem_name).as_str()],
            |output| {
                println!("saving the Message output:");
                run_and_print_output(vec![format!(r#"@echo {} >> N:\{}\processedMessage.txt"#, output, tem_name).as_str()]).unwrap();
                // TODO do things with the message file lines
                let mut tokens = output.split(": ");
                match tokens.next() {
                    Some("Copied") => {
                        let section = tokens.next().unwrap().split(" ").next().unwrap();
                        // handle core builds with TEMCoreBuildFast
                        if section.starts_with("core") {
                            spawn_core_build_thread(section.to_string());
                        }
                        // handle RC3 builds by copying to rawdata, importing and building
                        else {
                            println!("{}", section);
                            // copy to rawdata, automatically build to its own section
                            // (but do this in another thread, so notifications still pipe to Slack for other messages)
                            spawn_copy_and_build_thread(section.to_string());
                        }
                    },
                    // also extract the section like "Copied" does, then downscale its overview (with rust imagemagick? :D) and send it to slack
                    Some("Overview") => {
                        let section = tokens.next().unwrap().split(" ").next().unwrap();
                        let overview_path = format!(r#"N:\{}\overview{}.jpg"#, tem_name, section);
                        let small_overview_path = format!(r#"N:\{}\overview{}-small.jpg"#, tem_name, section);
                        run_chain_and_save_output(
                            vec![
                                vec!["magick", "convert", &overview_path, "-resize", "500x500", &small_overview_path],
                                rito_image(&small_overview_path),
                            ],
                            rito(format!("overview -> slack failed for {}", section).as_str())).unwrap();
                    },
                    Some(other_label) => {
                        println!("{}", other_label);
                        run_and_print_output(rito(output.as_str())).unwrap();
                    },
                    None => ()
                }

                println!("{}", output);
            },
            60,
            rito(format!("bob the builder {} thread failed", tem_name).as_str())).unwrap();
        }
    )
}

fn rito(message: &str) -> Vec<&str> {
    vec!["rito", "--slack", "tem-bot", message]
}

fn rito_image(path: &str) -> Vec<&str> {
    vec!["rito", "--slack_image", "tem-bot", path]
}

fn main() {
    let argv: Vec<_> = env::args().map(|v| v.to_owned()).collect();
    let mut argv: Vec<_> = argv.iter().map(|s| &**s).collect();
    argv.drain(0 .. 1);

    match argv.as_slice() {
        // When run with the `queue` subcommand, queue commands from a text file and save their outputs:
        ["queue", queue_file] => {
            println!("called as queue");

            let queue = lines_from_file(queue_file);
            let queue: Vec<Vec<&str>> = queue.iter().map(|s| s.split("~").collect()).collect();
            // The file has to tokenize command arguments like~this~"even though it's weird"
            run_chain_and_save_output(queue, rito("bob queue failed")).unwrap();
        },
        // Default behavior: monitor for TEM events and run data copies/builds
        [] => {
            let t1 = spawn_tem_message_reader_thread("TEM1");
            let t2 = spawn_tem_message_reader_thread("TEM2");

            t1.join().unwrap();
            t2.join().unwrap();
        },
        _ => {
            panic!("bad invocation of bob");
        }
    };
}