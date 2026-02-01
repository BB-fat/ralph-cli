use std::io;

/// Custom result type for Ralph CLI operations
pub type RalphResult<T> = Result<T, RalphError>;

/// Custom error type for Ralph CLI
#[derive(Debug)]
pub enum RalphError {
    Io(io::Error),
    Dialoguer(dialoguer::Error),
    Other(String),
}

impl std::fmt::Display for RalphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RalphError::Io(e) => write!(f, "IO error: {}", e),
            RalphError::Dialoguer(e) => write!(f, "Dialog error: {}", e),
            RalphError::Other(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for RalphError {}

impl From<io::Error> for RalphError {
    fn from(e: io::Error) -> Self {
        RalphError::Io(e)
    }
}

impl From<dialoguer::Error> for RalphError {
    fn from(e: dialoguer::Error) -> Self {
        RalphError::Dialoguer(e)
    }
}
