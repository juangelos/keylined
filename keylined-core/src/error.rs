//! Error types for Keyline

use thiserror::Error;

/// Custom error type for Keyline
#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Command execution error: {0}")]
    Command(String),

    #[error("UI error: {0}")]
    Ui(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Logging error: {0}")]
    Logging(String),
}

/// Result type for Keyline operations
pub type Result<T> = std::result::Result<T, Error>;
