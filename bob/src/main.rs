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
use std::sync::{Arc, Mutex};

const DROPBOX_DIR: &str = r#"D:\DROPBOX"#;
const DROPBOX_LINK_DIR: &str = r#"\\OpR-Marc-RC2\Data\DROPBOX"#;
const BUILD_TARGET: &str = r#"W:\Volumes"#;
const PYTHON_ENV: &str = r#"C:\Python39\Scripts"#;
const RAW_DATA_DIR: &str = r#"\\OpR-Marc-Syn3\Data\RawData"#;

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
    match run_and_filter_output(&vec!["@echo Hello, world!"], |s| assert_eq!("Hello, world!", s)) {
        Ok(code) => assert_eq!(0, code),
        Err(err) => panic!("error {}", err)
    };
    match run_and_filter_output(&vec!["@echo Hello, world! 1>&2"], |s| assert_eq!("Hello, world! ", s)) {
        Ok(code) => assert_eq!(0, code),
        Err(err) => panic!("error {}", err)
    };
    match run_and_filter_output(&vec!["exit /b 1"], |s| println!("{}", s)) {
        Ok(code) => assert_eq!(1, code),
        Err(err) => panic!("error {}", err)
    };
    match run_and_filter_output(&vec!["@echo", "Hey", ">", "file.txt"], |_| {}) {
        Ok(code) => assert_eq!(0, code),
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

// TODO implement .bob fileformat for queues, that also include a command on error at the end & can be parsed to this
struct CommandChain<'a> {
    commands: Vec<Vec<&'a str>>,
    command_on_error: Vec<&'a str>,
}

fn run_chain_and_save_output(chain: CommandChain) -> Result<i32, Error> {
    let commands = chain.commands;
    let command_on_error = chain.command_on_error;
    for command in commands {
        // TODO uuids are confusing for this when they could be timestamps.
        let uuid = Uuid::new_v4();
        // TODO this requires first making the bob-output folder manually:
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
            Ok(error_code) if command[0] == "robocopy" && (error_code == 1 || error_code == 3) => continue, // Robocopy returns 1 on success. yikes
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

fn spawn_copy_and_build_thread(section: String, mutex: Arc<Mutex<i32>>) -> JoinHandle<()> {
    thread::spawn(move || {
        println!("waiting for mutex to build {}", section);
        let _ = mutex.lock().unwrap();
        println!("acquired mutex to build {}", section);
        let temp_volume_dir = format!(r#"{}\RC3{}"#, BUILD_TARGET, section);
        let mosaic_report_dest = format!(r#"{}\MosaicReports\{}\MosaicReport.html"#, DROPBOX_LINK_DIR, section);
        let queue_file_dest = format!(r#"{}\queue{}.txt"#, PYTHON_ENV, section);
        run_chain_and_save_output(
            CommandChain {
                commands: vec![
                    vec![
                        "RC3Import",
                        temp_volume_dir.as_str(),
                        format!(r#"{}\TEMXCopy\{}"#, DROPBOX_DIR, section).as_str(),
                    ],
                    vec![
                        "RC3Build",
                        temp_volume_dir.as_str(),
                    ],
                    // Automatic build finished with code 0. Prepare a queue file for the next build step.
                    vec![
                        "@echo",
                        format!(r#"copy-section-links~W:\Volumes\RC3\TEM\VolumeData.xml~{}\TEM\VolumeData.xml~bob-output"#, temp_volume_dir).as_str(),
                        ">>",
                        queue_file_dest.as_str(),
                    ],
                    vec![
                        "@echo",
                        format!(r#"robocopy~{}\TEM~W:\Volumes\RC3\TEM\~/MT:32~/LOG:RC3Robocopy.log~/MOVE~/nfl~/nc~/ns~/np~/E~/TEE~/R:3~/W:1~/REG~/DCOPY:DAT~/XO"#, temp_volume_dir).as_str(),
                        ">>",
                        queue_file_dest.as_str(),
                    ],
                    // TODO the queue file could also delete itself after it finishes 
                    // Move the automatic build's mosaicreport files to DROPBOX and send a link.
                    // If the mosaicreport files aren't there, the chain will fail (as it should) because that's
                    // a secondary indicator of build failure
                    robocopy_move(
                        format!(r#"{}\MosaicReport"#, temp_volume_dir).as_str(),
                        format!(r#"{}\MosaicReports\{}\MosaicReport\"#, DROPBOX_DIR, section).as_str()),
                    vec![
                        "move",
                        format!(r#"{}\MosaicReport.html"#, temp_volume_dir).as_str(),
                        mosaic_report_dest.as_str(),
                    ],
                    rito(format!("{} built automatically. Check {} and run `cd {} && bob queue {}` on Build1 if it looks good", section, mosaic_report_dest, PYTHON_ENV, queue_file_dest).as_str()),
                    robocopy_move(
                        format!(r#"{}\TEMXCopy\{}"#, DROPBOX_DIR, section).as_str(),
                        format!(r#"{}\RC3\{}\"#, RAW_DATA_DIR, section).as_str()),
                    rito(format!("{} copied to RawData", section).as_str()),
                ],
                command_on_error: rito(format!("automatic copy and build for {} failed", section).as_str())
        }).unwrap();
    })
}

fn spawn_core_build_thread(section: String, mutex: Arc<Mutex<i32>>) -> JoinHandle<()> {
    thread::spawn(move || {
        println!("waiting for mutex to build {}", section);
        let _ = mutex.lock().unwrap();
        println!("acquired mutex to build {}", section);
        let volume_dir = format!(r#"{}\TEMXCopy\{}volume"#, DROPBOX_DIR, section);
        let build_target = format!(r#"{}\{}"#, BUILD_TARGET, section);
        let section_dir = format!(r#"{}\TEMXCopy\{}"#, DROPBOX_DIR, section);
        run_chain_and_save_output(
            CommandChain {
                commands: vec![
                    // Put the section in a "volume" folder because TEMCoreBuildFast expects a volume, not a single section
                    vec![
                        "mkdir",
                        volume_dir.as_str(),
                    ],
                    vec![
                        "move",
                        section_dir.as_str(),
                        volume_dir.as_str(),
                    ],
                    vec![
                        "TEMCoreBuildFast",
                        build_target.as_str(),
                        volume_dir.as_str(),
                    ],
                    rito(format!("{0} built automatically.", section).as_str()),
                ],
                command_on_error: rito(format!("automatic core build for {0} failed", section).as_str())
            }).unwrap();
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

fn spawn_tem_message_reader_thread(tem_name: &'static str, mutex: Arc<Mutex<i32>>) -> JoinHandle<()> {
    thread::spawn(move || {
        run_on_interval_and_filter_output(
            vec![format!(r#"type N:\{0}\message.txt && break>N:\{0}\message.txt"#, tem_name).as_str()],
            |output| {
                println!("saving the Message output:");
                run_and_print_output(vec![format!(r#"@echo {} >> N:\{}\processedMessage.txt"#, output, tem_name).as_str()]).unwrap();
                let mut tokens = output.split(": ");
                match tokens.next() {
                    Some("Copied") => {
                        let section = tokens.next().unwrap().split(" ").next().unwrap();
                        // handle core builds with TEMCoreBuildFast
                        if section.starts_with("core") {
                            spawn_core_build_thread(section.to_string(), Arc::clone(&mutex));
                        }
                        // handle RC3 builds by copying to rawdata, importing and building
                        else {
                            println!("{}", section);
                            // copy to rawdata, automatically build to its own section
                            // (but do this in another thread, so notifications still pipe to Slack for other messages)
                            spawn_copy_and_build_thread(section.to_string(), Arc::clone(&mutex));
                        }
                    },
                    // also extract the section like "Copied" does, then downscale its overview (with rust imagemagick? :D) and send it to slack
                    Some("Overview") => {
                        let section = tokens.next().unwrap().split(" ").next().unwrap();
                        let overview_path = format!(r#"N:\{}\overview{}.jpg"#, tem_name, section);
                        let small_overview_path = format!(r#"N:\{}\overview{}-small.jpg"#, tem_name, section);
                        run_chain_and_save_output(
                            CommandChain {
                                commands: vec![
                                    vec!["magick", "convert", &overview_path, "-resize", "500x500", &small_overview_path],
                                    rito_image(&small_overview_path),
                                ],

                                command_on_error: rito(format!("overview -> slack failed for {}", section).as_str())
                            }).unwrap();
                    },
                    // This case will be used even if there's no colon in the message
                    Some(other_label) => {
                        println!("{}", other_label);
                        run_and_print_output(rito(output.as_str())).unwrap();
                    },
                    // This case should never be used
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

fn robocopy_move<'a>(source: &'a str, dest: &'a str) -> Vec<&'a str> {
    vec![
        "robocopy",
        source,
        dest,
        "/MT:32",
        "/LOG:RC3Robocopy.log",
        "/MOVE",
        "/nfl",
        "/nc",
        "/ns",
        "/np",
        "/E",
        "/TEE",
        "/R:3",
        "/W:1",
        "/REG",
        "/DCOPY:DAT",
        "/XO",
    ]
}

fn main() {
    let argv: Vec<_> = env::args().map(|v| v.to_owned()).collect();
    let mut argv: Vec<_> = argv.iter().map(|s| &**s).collect();
    argv.drain(0 .. 1);

    match argv.as_slice() {
        // When run with the `queue` subcommand, queue commands from a text file and save their outputs:
        ["queue", queue_file] => {
            println!("called as queue");
            // TODO q as alias for queue
            // TODO allow passing arguments to a queue
            // TODO convert cmd files to queue.txt files
            // TODO allow queueing multiple queue files with varargs

            let queue = lines_from_file(queue_file);
            // TODO tokenize queue files by passing the lines through a filter that just prints each arg on a line
            let queue: Vec<Vec<&str>> = queue.iter().map(|line| line.split("~").map(|token| token.trim()).collect()).collect();
            // TODO The file has to tokenize command arguments like~this~"even though it's weird"
            run_chain_and_save_output(CommandChain {
                commands: queue, 
                command_on_error: rito("bob queue failed")
            }).unwrap();
        },
        // Default behavior: monitor for TEM events and run data copies/builds
        [] => {
            // Make a mutex to ensure only one build runs at a time:
            let mutex = Arc::new(Mutex::new(0));

            let t1 = spawn_tem_message_reader_thread("TEM1", Arc::clone(&mutex));
            let t2 = spawn_tem_message_reader_thread("TEM2", Arc::clone(&mutex));

            t1.join().unwrap();
            t2.join().unwrap();
        },
        _ => {
            panic!("bad invocation of bob");
        }
    };
}