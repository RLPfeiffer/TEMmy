use serde::{Serialize, Deserialize};
use std::path::Path;
use fs2::free_space;
extern crate fs_extra;
use fs_extra::dir::get_size;

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

    pub save_raw_data: bool,

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

        // Find if the data is in rawdata or TEMXCopy
        let path_in_temxcopy = format!(r#"{}\TEMXCopy\{}\{}"#, config.dropbox_dir, self.name, section);
        let path_in_raw_data = format!(r#"{}\{}\{}"#, config.raw_data_dir, self.name, section);
        let data_dir = if self.save_raw_data && Path::new(&path_in_raw_data).exists() {
            path_in_raw_data.clone()
        } else {
            path_in_temxcopy
        };

        let source_size = get_size(Path::new(&data_dir))?;
        let temp_volume_dir = format!(r#"{}\{}_temp\{}"#, config.build_target, self.name, section);
        let overflow_volume_dir = format!(r#"{}\{}_temp\{}"#, config.overflow_build_target, self.name, section);
        let available_space = free_space(Path::new(&temp_volume_dir));
        let enough_space = if let Ok(available) = available_space {
            if available > source_size { true } else { false }
        } else { false };
            
        let temp_volume_dir = if enough_space { temp_volume_dir } else { overflow_volume_dir };
        
        let mosaic_report_folder = format!(r#"{}\MosaicReports\{}\{}"#, config.dropbox_link_dir, self.name, section);
        let mosaic_report_dest = format!(r#"{}\MosaicReports\{}\{}\MosaicReport.html"#, config.dropbox_link_dir, self.name, section);

        // Volumes with 2-step import/build
        if let Some(import_script) = &self.import_script {
            // Example: RC3Import D:\Volumes\RC3_temp\0001 Y:\DROPBOX\TEMXCopy\RC3\0001
            commands.push(vec![
                import_script.clone(),
                temp_volume_dir.clone(),
                data_dir.clone(),
            ]);

            // Example: RC3Build D:\Volumes\RC3_temp\0001
            commands.push(vec![
                self.build_script.clone(),
                temp_volume_dir.clone(),
            ]);
        }
        // Volumes with a 1-step BuildFast script:
        else {
            // Example: TEMCoreBuildFast D:\Volumes\JeanneAllSections_temp\110001 Y:\DROPBOX\TEMXCopy\JeanneAllSections 110001
            commands.push(vec![
                self.build_script.clone(),
                temp_volume_dir.clone(),
                Path::new(&data_dir).parent().unwrap().to_string_lossy().to_string(),
                section.clone(),
            ])
        }

        // Automatic build finished with code 0 and no fatal error messages. 

        // Copy the automatic build's mosaicreport files to DROPBOX and send the mosaic overview image to Slack.
        // If the mosaicreport files aren't there, the chain will fail (as it should) because that's
        // a secondary indicator of build failure
        commands.push(robocopy_copy(
            format!(r#"{}\MosaicReport"#, temp_volume_dir.clone()),
            mosaic_report_folder.clone()));
        commands.push(vec![
            "copy".to_string(),
            format!(r#"{}\MosaicReport.html"#, temp_volume_dir),
            mosaic_report_dest.clone(),
        ]);
        commands.push(vec![
            "send-first-mosaic-overview".to_string(),
            mosaic_report_folder
        ]);

        // Send the pixel intensity histogram to Slack so we can decide whether to do ContrastOverrides:
        commands.push(rito_image(format!(r"{}\Histogram.png", data_dir)));

        // Move the data to Rawdata if it isn't already there
        if self.save_raw_data && data_dir != path_in_raw_data {
            commands.push(robocopy_move(
                        data_dir,
                        path_in_raw_data));
            // Notify the TEMs that they can clear the original data files:
            commands.push(rito_text_file(
                format!(r#"{}\TEMXCopy\rawdata.txt"#, config.dropbox_dir),
                format!("{}/{} copied to RawData", self.name, section)));
        }
        
        commands.push(rito(format!("{0} {1} built automatically. Run `Merge: {0} {1}` or click Merge on the web control panel if it looks good. Full MosaicReport: {2} ", self.name, section, mosaic_report_dest)));

        Ok(CommandChain {
            commands: commands,
            folders_to_lock: vec![temp_volume_dir.clone()],
            label: format!("automatic build for {} {}", self.name, section)
        })
    }

    pub fn merge_chain(&self, section:String) -> BobResult<CommandChain> {
        let temp_volume_dir = find_temp_volume(self.name.clone(), section.clone());
        
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
        commands.push(vec![
            self.optimize_tiles_script.clone(),
            self.path.clone(),
            section.clone()
        ]);
        
        // If an align script is given, use it
        if let Some(align_script) = &self.align_script {
            commands.push(vec![
                align_script.clone(),
                self.path.clone(),
            ])
        }
        
        Ok(CommandChain {
            commands: commands,
            folders_to_lock: vec![self.path.clone(), temp_volume_dir.clone()],
            label: format!("Automatic merge {} into {}", section, self.name.clone())
        })
    }

    pub fn fixmosaic_chain(&self, section:String, stage:bool) -> BobResult<CommandChain> {
        let temp_volume_dir = find_temp_volume(self.name.clone(), section.clone());

        Ok(CommandChain {
            commands: vec![
                // Delete the bad mosaic file:
                vec![
                    "del".to_string(),
                    format!(r#"{}\TEM\{}\TEM\{}"#, temp_volume_dir.clone(), section.clone(), self.mosaic_file.clone())
                ],
                vec![
                    if stage {
                        self.fixmosaic_stage_script.clone()
                    } else {
                        self.fixmosaic_script.clone()
                    },
                    temp_volume_dir.clone(),
                    section.clone(),
                ],
                rito(format!("Automatic FixMosaic finished for {} {}", self.name.clone(), section.clone())),
                vec![
                    "send-first-mosaic-overview".to_string(),
                    temp_volume_dir.clone()
                ],
            ],
            folders_to_lock: vec![temp_volume_dir.clone()],
            label: format!("automatic fixmosaic for {} {}", self.name.clone(), section.clone())
        })
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