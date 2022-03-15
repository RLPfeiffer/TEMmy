use std::path::Path;
use fs2::free_space;
extern crate fs_extra;
use fs_extra::dir::get_size;

use crate::robocopy::*;
use crate::rito::*;
use crate::config::*;
use crate::CommandChain;
use crate::run::*;
use crate::run::ShouldPrint::*;

pub fn rc3_build_chain(section: String, is_rebuild: bool) -> Option<CommandChain> {
    let config = config_from_yaml();

    let section_parts = section.split("_").collect::<Vec<&str>>();

    match &section_parts[..] {
        ["Jones", "RC3", section_number] => {
            let mut commands: Vec<Vec<String>> = vec![];
            let source = if is_rebuild {
                format!(r#"{}\RC3\{}"#, config.raw_data_dir, section_number)
            } else {
                // Rename the folder to just the section number so RC3Import knows it contains only 1 section:
                commands.push(vec![
                    "rename".to_string(),
                    format!(r#"{}\TEMXCopy\{}"#, config.dropbox_dir, section),
                    section_number.to_string(),
                ]);
                format!(r#"{}\TEMXCopy\{}"#, config.dropbox_dir, section_number)
            };
            let source_size = get_size(Path::new(&source));
            let temp_volume_dir = format!(r#"{}\RC3{}"#, config.build_target, section_number);
            let overflow_volume_dir = format!(r#"{}\RC3{}"#, config.overflow_build_target, section_number);
            let available_space = free_space(Path::new(&temp_volume_dir));
            let enough_space = if let Ok(available) = available_space {
                if let Ok(size) = source_size {
                    if available > size { true } else { false }
                } else {
                    false
                }
            } else { false };
                
            let temp_volume_dir = if enough_space { temp_volume_dir } else { overflow_volume_dir };
            
            let mosaic_report_folder = format!(r#"{}\MosaicReports\{}"#, config.dropbox_link_dir, section_number);
            let mosaic_report_dest = format!(r#"{}\MosaicReports\{}\MosaicReport.html"#, config.dropbox_link_dir, section_number);
            
            let mut rest_commands = vec![
                vec![
                    "RC3Import".to_string(),
                    temp_volume_dir.clone(),
                    source,
                ],
                vec![
                    "RC3Build".to_string(),
                    temp_volume_dir.clone()
                ],
                // Automatic build finished with code 0. 

                // TODO check that a tileset was generated.
                // TODO sent the mosaicreport overview to slack

                // Copy the automatic build's mosaicreport files to DROPBOX and send a link.
                // If the mosaicreport files aren't there, the chain will fail (as it should) because that's
                // a secondary indicator of build failure
                robocopy_copy(
                    format!(r#"{}\MosaicReport"#, temp_volume_dir.clone()),
                    format!(r#"{}\MosaicReports\{}\MosaicReport\"#, config.dropbox_dir, section_number)),
                vec![
                    "copy".to_string(),
                    format!(r#"{}\MosaicReport.html"#, temp_volume_dir),
                    mosaic_report_dest.clone(),
                ],
                vec![
                    "send-first-mosaic-overview".to_string(),
                    mosaic_report_folder
                ],
            ];
            commands.append(&mut rest_commands);

            let data_path = format!(r#"{}\TEMXCopy\{}"#, config.dropbox_dir, section_number);
            let rawdata_path = format!(r#"{}\RC3\{}\"#, config.raw_data_dir, section_number);
            commands.push(rito_image(format!(r"{}\Histogram.png", if is_rebuild { rawdata_path.clone() } else { data_path.clone() })));
            if !is_rebuild {
                commands.push(robocopy_move(
                        data_path,
                        rawdata_path));
                commands.push(rito_text_file(
                    format!(r#"{}\TEMXCopy\rawdata.txt"#, config.dropbox_dir),
                    format!("{} copied to RawData", section)));
            }

            commands.push(rito(format!("{0} built automatically. Run `Merge: {0}` or click Merge on the web control panel if it looks good. Full MosaicReport: {1} ", section_number, mosaic_report_dest)));

            Some(CommandChain {
                commands: commands,
                label: if is_rebuild {
                    format!("automatic rebuild for RC3 {}", section_number)
                } else {
                    format!("automatic copy and build for RC3 {}", section_number)
                }
            })
        },
        _ => {
            run_warn(rito(format!("{0} should be named with pattern Jones_RC3_[section] and was not built automatically", section)), Print);
            None
        },
    }
}

pub fn rc3_fixmosaic(section:String) -> CommandChain {
    let config = config_from_yaml();

    let temp_volume_dir = find_temp_volume(section.clone());

    CommandChain {
        commands:
            vec![
                vec![
                    "del".to_string(),
                    format!(r#"{}\TEM\{}\TEM\Translated_Prune_Max0.5.mosaic"#, temp_volume_dir.clone(), section.clone())
                ],
                vec![
                    "RC3FixMosaic".to_string(),
                    temp_volume_dir.clone(),
                    section.clone()
                ],
                rito(format!("RC3FixMosaic finished for {}", section.clone())),
                vec![
                    "send-first-mosaic-overview".to_string(),
                    temp_volume_dir.clone()
                ],
            ],
        label:
            format!("automatic fixmosaic for RC3 {}", section)
    }
}

fn find_temp_volume(section:String) -> String {
    let config = config_from_yaml();

    let temp_volume_dir = format!(r#"{}\RC3{}"#, config.build_target, section);
    let overflow_volume_dir = format!(r#"{}\RC3{}"#, config.overflow_build_target, section);
    if Path::new(&temp_volume_dir).exists() {
        temp_volume_dir
    } else {
        overflow_volume_dir
    }
}

fn merge_commands(section:String) -> Vec<Vec<String>> {
    let config = config_from_yaml();

    let temp_volume_dir = find_temp_volume(section.clone());

    vec![
        vec![    
            "copy-section-links".to_string(),
            r#"W:\Volumes\RC3\TEM\VolumeData.xml"#.to_string(), // TODO this is RC3 hard-coded
            format!(r#"{}\TEM\VolumeData.xml"#, temp_volume_dir),
            "bob-output".to_string()
        ],
        robocopy_move(
            format!(r#"{}\TEM"#, temp_volume_dir),
            r#"W:\Volumes\RC3\TEM\"#.to_string()),
        vec![
            "nornir-build".to_string(),
            r#"W:\Volumes\RC3"#.to_string(), 
            "CreateVikingXML".to_string(),
            "-OutputFile".to_string(),
            "Mosaic".to_string()
        ],
        // Delete the temp volume
        vec![
            "rmdir".to_string(),
            "/S".to_string(),
            "/Q".to_string(),
            temp_volume_dir
        ],
        vec![
            "RC3OptimizeTiles".to_string(),
            r#"W:\Volumes\RC3"#.to_string(),
            section.clone()
        ],

    ]
}

pub fn rc3_merge_chain(section: String) -> CommandChain {
    CommandChain {
        commands: merge_commands(section.clone()),
        label: format!("automatic merge for {} into RC3", section)
    }
}