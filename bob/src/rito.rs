pub fn rito(message: String) -> Vec<String> {
    vec!["rito".to_string(), "--slack".to_string(), "tem-bot".to_string(), message]
}

pub fn rito_image(path: String) -> Vec<String> {
    vec!["rito".to_string(), "--slack_image".to_string(), "tem-bot".to_string(), path]
}

pub fn rito_file(path: String) -> Vec<String> {
    vec!["rito".to_string(), "--slack_file".to_string(), "tem-bot".to_string(), path]
}