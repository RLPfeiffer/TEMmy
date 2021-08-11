// These functions return Commands that run robocopy

#[derive(Debug)]
pub enum RobocopyType {
    Move,
    Copy,
}

pub fn robocopy<'a>(typ: RobocopyType, source: String, dest: String) -> Vec<String> {
    let mut command = vec![
        "robocopy".to_string(),
        source,
        dest,
        "/MT:32".to_string(),
        "/LOG:RC3Robocopy.log".to_string(),
        "/nfl".to_string(),
        "/nc".to_string(),
        "/ns".to_string(),
        "/np".to_string(),
        "/E".to_string(),
        "/TEE".to_string(),
        "/R:3".to_string(),
        "/W:1".to_string(),
        "/REG".to_string(),
        "/DCOPY:DAT".to_string(),
        "/XO".to_string(),
    ];
    match typ {
        RobocopyType::Move => {
            command.push("/MOVE".to_string());
        },
        RobocopyType::Copy => (),
    }
    command
}

pub fn robocopy_move<'a>(source: String, dest: String) -> Vec<String> {
    robocopy(RobocopyType::Move, source, dest)
}

pub fn robocopy_copy<'a>(source: String, dest: String) -> Vec<String> {
    robocopy(RobocopyType::Copy, source, dest)
}