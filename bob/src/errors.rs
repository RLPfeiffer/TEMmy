use thiserror::Error;

use crate::CommandChain;

#[derive(Error, Debug)]
pub enum BobError {
    #[error("Exit code {0}")]
    BadExitCode(i32),

    #[error(transparent)]
    ParseExitCodeError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
    
    #[error("Fatal error {0}")]
    FatalRegex(String),

    #[error(transparent)]
    SendErrorString(#[from] std::sync::mpsc::SendError<String>),

    #[error(transparent)]
    SendErrorCommandChain(#[from] std::sync::mpsc::SendError<CommandChain>),
}

pub type BobResult<T> = Result<T, BobError>;