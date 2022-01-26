// These functions return Commands that send messages and files
// through our slack bot

use crate::run::Command;
use crate::config_from_yaml;

pub fn rito(message: String) -> Command {
    vec!["rito".to_string(), "--slack".to_string(), "tem-bot".to_string(), message]
}

pub fn rito_image(path: String) -> Command {
    vec!["rito".to_string(), "--slack_image".to_string(), "tem-bot".to_string(), path]
}

pub fn rito_file(path: String) -> Command {
    vec!["rito".to_string(), "--slack_file".to_string(), "tem-bot".to_string(), path]
}

// rito-get --slack uses the https://api.slack.com/methods/conversations.history method,
// which as of 1/26/22 is a Tier 3 method allowing 50+ calls per minute.
// We want Bob to be open to a response throughout the Tuesday of a long weekend
// without making Slack mad at us.
pub fn rito_get(pattern: String) -> Command {
    let timeout_days = 4;
    let timeout_min = timeout_days
                        * 24 // hours per day
                        * 60; // minutes per hour
    let timeout_sec = timeout_min
                        * 60; // seconds per minute
    let num_checks = 48 * timeout_days; // Twice per hour

    let potential_get_threads = config_from_yaml().worker_threads;
    assert!(num_checks / timeout_min < 50 / potential_get_threads, "rito-get could exceed slack API rate limit. Fix rito_get() in rito.rs");

    vec!["rito-get".to_string(), "--slack".to_string(), "tem-bot".to_string(), pattern, "--timeout".to_string(), format!("{}", timeout_sec), "--num-checks".to_string(), format!("{}", num_checks)]
}

pub fn rito_text_file(path:String, message:String) -> Command {
    vec!["rito".to_string(), "--file".to_string(), path, message]
}