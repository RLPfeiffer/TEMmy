// These functions return Commands that send messages and files
// through our slack bot

use crate::run::Command;

pub fn rito(message: String) -> Command {
    vec!["rito".to_string(), "--slack".to_string(), "tem-bot".to_string(), message]
}

pub fn rito_image(path: String) -> Command {
    vec!["rito".to_string(), "--slack_image".to_string(), "tem-bot".to_string(), path]
}

pub fn rito_file(path: String) -> Command {
    vec!["rito".to_string(), "--slack_file".to_string(), "tem-bot".to_string(), path]
}