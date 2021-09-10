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
            let temp_volume_dir = format!(r#"{}\RC3{}"#, config.build_target, section_number);
            let mosaic_report_dest = format!(r#"{}\MosaicReports\{}\MosaicReport.html"#, config.dropbox_link_dir, section_number);
            let source = if is_rebuild {
                format!(r#"{}\RC3\{}"#, config.raw_data_dir, section_number)
            } else {
                format!(r#"{}\TEMXCopy\{}"#, config.dropbox_dir, section)
            };
            let mut commands = vec![
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
                rito(format!("{0} built automatically. Check {1} and run `Merge: {0}` if it looks good", section_number, mosaic_report_dest)),
            ];

            if !is_rebuild {
                commands.push(robocopy_move(
                        format!(r#"{}\TEMXCopy\{}"#, config.dropbox_dir, section),
                        format!(r#"{}\RC3\{}\"#, config.raw_data_dir, section_number)));
                commands.push(rito(format!("{} copied to RawData", section_number)));
            }

            Some(CommandChain {
                commands: commands,
                label: format!("automatic copy and build for RC3 {}", section_number)
            })
        },
        _ => {
            run_warn(rito(format!("{0} should be named with pattern Jones_RC3_[section] and was not built automatically", section)), Print);
            None
        },
    }
}

pub fn rc3_merge_chain(section: String) -> CommandChain {
    let config = config_from_yaml();

    let temp_volume_dir = format!(r#"{}\RC3{}"#, config.build_target, section);

    CommandChain {
        commands: vec![
            vec![    
                "copy-section-links".to_string(),
                r#"W:\Volumes\RC3\TEM\VolumeData.xml"#.to_string(), // TODO this is RC3 hard-coded
                format!(r#"{}\TEM\VolumeData.xml"#, temp_volume_dir),
                "bob-output".to_string()
            ],
            robocopy_move(
                format!(r#"{}\TEM"#, temp_volume_dir),
                r#"W:\Volumes\RC3\TEM\"#.to_string()),
            // Delete the temp volume
            vec![
                "rmdir".to_string(),
                "/S".to_string(),
                "/Q".to_string(),
                temp_volume_dir
            ],
        ],
        label: format!("automatic merge for {} into RC3", section)
    }
}