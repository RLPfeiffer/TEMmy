use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub dropbox_dir: String,
    pub dropbox_link_dir: String,
    pub build_target: String,
    pub raw_data_dir: String,
    pub notification_dir: String,
    pub core_deployment_dir: String,
    pub worker_threads: i64,
    pub process_tem_output: bool,
    pub automatic_builds: bool,
    pub junk_outputs: Vec<String>,
    pub fatal_errors: Vec<String>,
}

pub fn config_from_yaml() -> Config {
    let yaml_str = fs::read_to_string("bob-config.yaml").expect("bob requires a bob-config.yaml file");
    serde_yaml::from_str(&yaml_str).unwrap()
    // TODO have a list of volumes in the yaml file and let them define
    // import/build/merge/align script chains
}