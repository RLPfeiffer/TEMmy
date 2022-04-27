use serde::{Serialize, Deserialize};
use std::path::Path;
use fs2::free_space;
extern crate fs_extra;
use fs_extra::dir::get_size;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::robocopy::*;
use crate::rito::*;
use crate::config::*;
use crate::CommandChain;
use crate::run::*;
use crate::errors::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    pub name: String,
    pub path: String,

    pub raw_data_dir: Option<String>,

    pub import_script: Option<String>,
    pub build_script: String,
    pub optimize_tiles_script:String,
    pub align_script: Option<String>,
    pub fixmosaic_script: String,
    pub fixmosaic_stage_script: String,
    pub mosaic_file: String,
}

impl Volume {
    pub fn build_chain(&self, section:String) -> BobResult<CommandChain> {
        let config = config_from_yaml();
        
        let mut commands = Vec::<Command>::new();

        let mut path_in_raw_data = None;
        let data_dir = self.find_data_dir(section.clone(), &mut path_in_raw_data);

        let source_size = get_size(Path::new(&data_dir))?;
        let temp_volume_dir = format!(r#"{}\{}_temp\{}"#, config.build_target, self.name, section);
        let overflow_volume_dir = format!(r#"{}\{}_temp\{}"#, config.overflow_build_target, self.name, section);
        let available_space = free_space(Path::new(&temp_volume_dir));
        let enough_space = if let Ok(available) = available_space {
            if available > source_size { true } else { false }
        } else { false };
            
        let temp_volume_dir = if enough_space { temp_volume_dir } else { overflow_volume_dir };
        
        // Volumes with 2-step import/build
        if let Some(import_script) = &self.import_script {
            // Example: RC3Import D:\Volumes\RC3_temp\0001 Y:\DROPBOX\TEMXCopy\RC3\0001
            commands.extend(commands_from_cmd_file(import_script.clone(), vec![temp_volume_dir.clone(), data_dir.clone()])?);

            // Example: RC3Build D:\Volumes\RC3_temp\0001
            commands.extend(commands_from_cmd_file(self.build_script.clone(), vec![temp_volume_dir.clone()])?);
        }
        // Volumes with a 1-step BuildFast script:
        else {
            // Example: TEMCoreBuildFast D:\Volumes\JeanneAllSections_temp\110001 Y:\DROPBOX\TEMXCopy\JeanneAllSections\110001
            commands.extend(commands_from_cmd_file(self.build_script.clone(), vec![
                temp_volume_dir.clone(),
                data_dir.clone(),
            ])?);
       }

        // Automatic build finished with code 0 and no fatal error messages. 

       commands.push(vec![
            "send-first-mosaic-overview".to_string(),
            temp_volume_dir.clone(),
        ]);

        // Send the pixel intensity histogram to Slack so we can decide whether to do ContrastOverrides:
        commands.push(rito_image(format!(r"{}\Histogram.png", data_dir)));

        // Move the data to Rawdata if it isn't already there
        if let Some(path_in_raw_data) = path_in_raw_data {
            if data_dir != path_in_raw_data {
                commands.push(robocopy_move(
                            data_dir,
                            path_in_raw_data));
                // Notify the TEMs that they can clear the original data files:
                commands.push(rito_text_file(
                    format!(r#"{}\TEMXCopy\rawdata.txt"#, config.dropbox_dir),
                    format!("{}/{} copied to RawData", self.name, section)));
            }
        }
        
        commands.push(rito(format!("{0} {1} built automatically. Run `Merge: {0} {1}` or click Merge on the web control panel if it looks good.", self.name, section)));

        Ok(CommandChain {
            commands: commands,
            folders_to_lock: vec![temp_volume_dir.clone()],
            label: format!("automatic build for {} {}", self.name, section)
        })
    }

    pub fn deploy_chain(&self, section:String) -> BobResult<CommandChain> {
        let config = config_from_yaml();
        let mut host_url = "".to_string();
        for host in config.hosts {
            if self.path.starts_with(&host.drive_letter) {
                host_url = host.url.clone();
            }
        }
        if host_url.len() == 0 {
            return Err(BobError::Bob("Tried to deploy volume without a defined host for its drive letter".to_string()));
        }

        let temp_volume_dir = find_temp_volume(self.name.clone(), section.clone());
        let mut commands = Vec::<Command>::new();
        commands.push(robocopy_move(
            temp_volume_dir.clone(),
            format!(r#"{}\"#, self.path.clone())));

        commands.push(
            vec![
                "add-volume-path".to_string(),
                format!(r#"{}\Mosaic.VikingXML"#, self.path.clone()),
                self.name.clone(),
                host_url.clone(),
                "bob-output".to_string() // backup dir for Mosaic.VikingXML files
            ]);

        commands.extend(commands_from_cmd_file(self.optimize_tiles_script.clone(), vec![
            self.path.clone(),
            section.clone()
        ])?);
 
        Ok(CommandChain {
            commands: commands,
            folders_to_lock: vec![self.path.clone(), temp_volume_dir.clone()],
            label: format!("Automatic deploy {} as first section of volume {}", section, self.name.clone())
        })
    }

    pub fn merge_chain(&self, section:String) -> BobResult<CommandChain> {
        let temp_volume_dir = find_temp_volume(self.name.clone(), section.clone());
        
        // TODO if the volume doesn't exist yet, use this first section as the basis for the volume, and add the host info to Mosaic.VikingXML
        let volume_path = Path::new(&self.path);
        if !volume_path.exists() {
            fs::create_dir_all(volume_path)?;
            return self.deploy_chain(section);
        }

        let mut commands = Vec::<Command>::new();
        
        // Uncomment for testing a failed command/manual unlock:
        // commands.push(vec!["exit".to_string(), "/b".to_string(), "1".to_string()]);

        commands.push(vec![
            "copy-section-links".to_string(),
            format!(r#"{}\TEM\VolumeData.xml"#, self.path.clone()),
            format!(r#"{}\TEM\VolumeData.xml"#, temp_volume_dir),
            "bob-output".to_string()
        ]);

        commands.push(robocopy_move(
            format!(r#"{}\TEM"#, temp_volume_dir),
            format!(r#"{}\TEM\"#, self.path.clone())));
        
        commands.push(vec![
            "nornir-build".to_string(),
            self.path.clone(), 
            "CreateVikingXML".to_string(),
            "-OutputFile".to_string(),
            "Mosaic".to_string()
        ]);
        
        // Delete the temp volume
        commands.push(vec![
            "rmdir".to_string(),
            "/S".to_string(),
            "/Q".to_string(),
            temp_volume_dir.clone(),
        ]);
        commands.extend(commands_from_cmd_file(self.optimize_tiles_script.clone(), vec![
            self.path.clone(),
            section.clone()
        ])?);
        
        // Old behavior: If an align script is given, use it
        // This was disabled because aligns can take so long.
        // If it ever needs to be re-enabled it should use commands.extend(commands_from_cmd_file)
        /*if let Some(align_script) = &self.align_script {
            commands.push(vec![
                align_script.clone(),
                self.path.clone(),
            ])
        }*/
        
        Ok(CommandChain {
            commands: commands,
            folders_to_lock: vec![self.path.clone(), temp_volume_dir.clone()],
            label: format!("Automatic merge {} into {}", section, self.name.clone())
        })
    }

    pub fn fixmosaic_chain(&self, section:String, stage:bool) -> BobResult<CommandChain> {
        let temp_volume_dir = find_temp_volume(self.name.clone(), section.clone());

        let mut commands = vec![
            // Delete the bad mosaic file:
            vec![
                "del".to_string(),
                format!(r#"{}\TEM\{}\TEM\{}"#, temp_volume_dir.clone(), section.clone(), self.mosaic_file.clone())
            ],
        ];

        commands.extend(
            commands_from_cmd_file(
                if stage {
                    self.fixmosaic_stage_script.clone()
                } else {
                    self.fixmosaic_script.clone()
                }, vec![
                    temp_volume_dir.clone(),
                    section.clone(),
                ])?);

        commands.push(rito(format!("Automatic FixMosaic finished for {} {}", self.name.clone(), section.clone())));
        commands.push(vec![
            "send-first-mosaic-overview".to_string(),
            temp_volume_dir.clone()
        ]);

        Ok(CommandChain {
            commands: commands,
            folders_to_lock: vec![temp_volume_dir.clone()],
            label: format!("automatic fixmosaic for {} {}", self.name.clone(), section.clone())
        })
    }

    pub fn contrast_overrides_chain(&self, section:String, min:u64, max:u64) -> BobResult<CommandChain> {
        let mut path_in_raw_data = None;
        let data_dir = self.find_data_dir(section.clone(), &mut path_in_raw_data);

        let path = Path::new(&data_dir).join("ContrastOverrides.txt");
        println!("{:?}", path);
        let overrides_line = format!("{} {} {} 1.0", section.clone(), min, max);

        // Return a command chain that is basically just a build chain:
        let build_chain = self.build_chain(section.clone())?;

        // With this done before it starts:
        // Write the contrast override options to the ContrastOverrides file
        let mut commands = build_chain.commands;

        commands.insert(0, vec![format!("@echo {} >> {}", overrides_line, path.to_string_lossy().to_string())]);
        commands.insert(0, vec![format!("@echo >> {}", path.to_string_lossy().to_string())]);

        Ok(CommandChain {
            commands: commands,
            label: format!("Contrast overrides for {} {}", self.name.clone(), section.clone()),
            folders_to_lock: build_chain.folders_to_lock
        })
    }

    // Find if the data for a section is in rawdata or TEMXCopy
    fn find_data_dir(&self, section:String, path_in_raw_data: &mut Option<String>) -> String {
        let config = config_from_yaml();
        let path_in_temxcopy = format!(r#"{}\TEMXCopy\{}\{}"#, config.dropbox_dir, self.name, section);
        let data_dir = if let Some(raw_data_dir) = &self.raw_data_dir {
            let _path_in_raw_data = format!(r#"{}\{}\{}"#, raw_data_dir, self.name, section);
            *path_in_raw_data = Some(_path_in_raw_data.clone());
            if Path::new(&_path_in_raw_data).exists() {
                _path_in_raw_data.clone()
            } else {
                path_in_temxcopy
            }
        } else {
            path_in_temxcopy
        };
        data_dir
    }
}

fn find_temp_volume(volume: String, section: String) -> String {
    let config = config_from_yaml();

    let temp_volume_dir = format!(r#"{}\{}_temp\{}"#, config.build_target, volume.clone(), section.clone());
    let overflow_volume_dir = format!(r#"{}\{}_temp\{}"#, config.overflow_build_target, volume, section);
    if Path::new(&temp_volume_dir).exists() {
        temp_volume_dir
    } else {
        overflow_volume_dir
    }
}

fn commands_from_cmd_file(file: String, args:Vec<String>) -> BobResult<Vec<Command>> {
    let file = if file.ends_with(".cmd") { file } else { format!("{}.cmd", file)};
    let cmd_str = fs::read_to_string(&file)?;
    let lines = cmd_str.split("\n");

    let mut bad_arg_expression = "".to_string();

    let commands = lines.map(
        |line| line.split(" ").map(
            |arg| if arg.starts_with("%") {
                let num = &arg[1..].trim();
                if let Ok(num) = num.parse::<usize>() {
                    args[num - 1].clone()
                } else {
                    bad_arg_expression = format!("Script file {} contains a % arg expression that bob cannot parse: {}", file, arg);
                    "".to_string()
                }
            } else {
                arg.trim().to_string()
            }).collect::<Vec<String>>()).collect::<Vec<Command>>();

    if bad_arg_expression.len() > 0 {
        run(rito(bad_arg_expression.clone()), crate::ShouldPrint::Print)?;
        Err(BobError::Bob(bad_arg_expression))
    } else {
        Ok(commands)
    }
}