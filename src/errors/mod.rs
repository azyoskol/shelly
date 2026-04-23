//! Error handling module for Shally

use std::fmt;

/// Main error type for Shally
#[derive(Debug)]
pub enum ShallyError {
    /// Configuration error
    Config(String),
    /// IO error
    Io(std::io::Error),
    /// Plugin error
    Plugin(String),
    /// AI service error
    AiService(String),
    /// History error
    History(String),
}

impl fmt::Display for ShallyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShallyError::Config(msg) => write!(f, "Configuration error: {}", msg),
            ShallyError::Io(err) => write!(f, "IO error: {}", err),
            ShallyError::Plugin(msg) => write!(f, "Plugin error: {}", msg),
            ShallyError::AiService(msg) => write!(f, "AI service error: {}", msg),
            ShallyError::History(msg) => write!(f, "History error: {}", msg),
        }
    }
}

impl std::error::Error for ShallyError {}

impl From<std::io::Error> for ShallyError {
    fn from(err: std::io::Error) -> Self {
        ShallyError::Io(err)
    }
}

/// Result type alias for Shally operations
pub type Result<T> = std::result::Result<T, ShallyError>;
