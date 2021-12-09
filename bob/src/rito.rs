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

pub fn rito_get(pattern: String) -> Command {
    vec!["rito-get".to_string(), "--slack".to_string(), "tem-bot".to_string(), pattern, "--timeout".to_string(), "10400".to_string()]
}

pub fn rito_text_file(path:String, message:String) -> Command {
    vec!["rito".to_string(), "--file".to_string(), path, message]
}