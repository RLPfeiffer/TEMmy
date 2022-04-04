use thiserror::Error;

use crate::CommandChain;

#[derive(Error, Debug)]
pub enum BobError {
    #[error("Exit code {0}")]
    BadExitCode(i32),

    #[error("{0} is None from {1}")]
    CommandNoneError(&'static str, String),

    #[error(transparent)]
    ParseExitCodeError(#[from] std::num::ParseIntError),

    #[error("No error code in exit message {0}")]
    BadExitMessage(String),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
    
    #[error("Fatal error {0}")]
    FatalRegex(String),

    #[error("Badly formed regex in bob-config.yaml: {0}")]
    RegexCompileError(#[from] regex::Error),

    #[error(transparent)]
    SendErrorString(#[from] std::sync::mpsc::SendError<String>),

    #[error(transparent)]
    SendErrorCommandChain(#[from] std::sync::mpsc::SendError<CommandChain>),

    #[error(transparent)]
    RecvError(#[from] std::sync::mpsc::RecvError),

    #[error(transparent)]
    ReadlineError(#[from] rustyline::error::ReadlineError),

    #[error(transparent)]
    SystemTimeError(#[from] std::time::SystemTimeError),
}

pub type BobResult<T> = Result<T, BobError>;