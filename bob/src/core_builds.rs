use crate::robocopy::*;
use crate::rito::*;
use crate::config::*;
use crate::CommandChain;
use crate::run::*;
use std::fs;
use crate::run::ShouldPrint::*;

pub fn core_build_chain(section: String, is_rebuild: bool) -> Option<CommandChain> {
    let config = config_from_yaml();

    let section_dir = format!(r#"{}\TEMXCopy\{}"#, config.dropbox_dir, section);
    let section_parts = section.split("_").collect::<Vec<&str>>();

    match &section_parts[..] {
        ["core", volume, section_number] => {
            let volume_dir = format!(r#"{}\TEMXCopy\{}"#, config.dropbox_dir, volume.clone());
            let mosaic_report_dest = format!(r#"{}\MosaicReports\{}\MosaicReport.html"#, config.dropbox_link_dir, volume.clone());
            let build_target = format!(r#"{}\{}"#, config.build_target, volume.clone());

            // If the volume dir doesn't exist, make it
            fs::create_dir_all(&volume_dir).unwrap();

            let mut commands = vec![
                // Run TEMCoreBuildFast
                vec![
                    "TEMCoreBuildFast".to_string(),
                    build_target.clone(),
                    volume_dir.clone(),
                ],

                // Copy the automatic build's mosaicreport files to DROPBOX and send a link.
                // If the mosaicreport files aren't there, the chain will fail (as it should) because that's
                // a secondary indicator of build failure
                robocopy_copy(
                    format!(r#"{}\MosaicReport"#, build_target.clone()),
                    format!(r#"{}\MosaicReports\{}\MosaicReport\"#, config.dropbox_dir, volume.clone())),
                vec![
                    "copy".to_string(),
                    format!(r#"{}\MosaicReport.html"#, build_target.clone()),
                    mosaic_report_dest.clone(),
                ],
                rito(format!("{0} built automatically. Check {1}, and if all sections have been built properly, run `Deploy: {0}` if it looks good", volume, mosaic_report_dest)),
            ];

            // Unless this a rebuild attempt, sections need to be moved to their volume directory:
            if !is_rebuild {
                // Move section into volume dir
                commands.insert(0, vec![
                    "move".to_string(),
                    section_dir,
                    format!(r#"{}\{}"#, volume_dir.clone(), section_number.clone()),
                ]);
            }

            Some(CommandChain {
                commands: commands,
                label: format!("automatic core build for {0}", section)
            })
        },
        _ => {
            run_warn(rito(format!("{0} should be named with pattern core_[volume]_[section] and was not built automatically", section)), Print);
            None
        },
    }
    
}

pub fn core_fixmosaic_stage(volume:String, sections: Vec<String>) -> CommandChain {
    let config = config_from_yaml();

    let mosaic_report_dest = format!(r#"{}\MosaicReports\{}\MosaicReport.html"#, config.dropbox_link_dir, volume.clone());
    let build_target = format!(r#"{}\{}"#, config.build_target, volume.clone());

    let mut commands = vec![];

    // Delete existing mosaics
    for section in &sections {
        let section_folder = format!("{:04}", section.parse::<i32>().unwrap()); // "420" -> "0420", "13345" -> "13345"
        commands.push(vec![
            "del".to_string(),
            format!(r#"{}\TEM\{}\TEM\Grid_Cel128_Mes8_Mes8_Thr0.25_it10_sp4.mosaic"#, build_target, section_folder),
        ]);
    }

    commands.append(&mut vec![
        vec![
            "TEMCoreBuildFixMosaic_Stage".to_string(),
            build_target.clone(),
            sections.join(","),
        ],
        // Copy the automatic build's mosaicreport files to DROPBOX and send a link.
        // If the mosaicreport files aren't there, the chain will fail (as it should) because that's
        // a secondary indicator of build failure
        robocopy_copy(
            format!(r#"{}\MosaicReport"#, build_target.clone()),
            format!(r#"{}\MosaicReports\{}\MosaicReport\"#, config.dropbox_dir, volume.clone())),
        vec![
            "copy".to_string(),
            format!(r#"{}\MosaicReport.html"#, build_target.clone()),
            mosaic_report_dest.clone(),
        ],
        rito(format!("{0} mosaic fixed with stage positions automatically. Check {1}, and if all sections have been built properly, run `Deploy: {0}` if it looks good", volume, mosaic_report_dest)),
    ]);

    CommandChain {
        commands: commands,
        label: format!("automatic fixmosaic_stage for {} {:?}", volume, sections)
    }
}

pub fn core_deploy_chain(volume: String) -> CommandChain {
    let config = config_from_yaml();

    let volume_dir = format!(r#"{}\{}"#, config.build_target.clone(), volume.clone());
    let deploy_dir = format!(r#"{}\{}"#, config.core_deployment_dir.clone(), volume.clone());

    CommandChain {
        commands: vec![
            robocopy_copy(
                volume_dir.clone(),
                format!(r#"{}\"#,deploy_dir.clone())),
            vec![
                "TEMCoreBuildOptimizeTiles".to_string(),
                deploy_dir.clone(),
            ],
            vec![
                "add-volume-path".to_string(),
                format!(r#"{}\Mosaic.VikingXML"#, deploy_dir.clone()),
                "bob-output".to_string() // backup dir for Mosaic.VikingXML files
            ],
            rito(format!(r#"{0} might be ready! Check http://storage1.connectomes.utah.edu/{0}/Mosaic.VikingXML in Viking"#, volume))
        ],
        label: format!("automatic core volume deployment for {0} to http://storage1.connectomes.utah.edu/{0}/Mosaic.VikingXML", volume)
    }
}