use serde::ser::StdError;
use std::fmt;
use std::io;
use thiserror::Error;
#[derive(Error, Debug)]
/// Errors handling failures to connect to the Redis instance
pub enum DBConnectorError {
    #[error("Redis error: {0}")]
    RedisConnectorError(#[from] redis::RedisError),
}
#[derive(Error, Debug)]
pub enum CliError {
    #[error("Unexpected: `{expected:?}` arguments found `{actual:?}` arguments")]
    UnexpectedNumberOfArguments { expected: usize, actual: usize },
    #[error("Empty argument provided at position {arg_position:?}")]
    EmptyArgument { arg_position: usize },
}
/// Errors that can occur when trying to resolve the position of an address from a file
#[derive(Error, Debug)]
pub enum AddressPositionError {
    #[error("no matching index found for address: {0}, value: {1}")]
    NoMatchingIndexForValue(String, i64),
    #[error("no matching address found: {0}")]
    NoMatchingAddress(String),
    #[error("no matching indices found for {address} with value: {value}")]
    NoMatchingIndices { address: String, value: String },
}
/// Errors that can occur while copying a file.
#[derive(Debug, Error)]
pub enum FileError {
    #[error("Failed to write file")]
    IoError(std::io::Error),
    #[error("File not found")]
    FileNotFound,
}

impl From<std::io::Error> for FileError {
    fn from(err: std::io::Error) -> Self {
        FileError::IoError(err)
    }
}
/// Errors that can occur while generating C++ files.
#[derive(Debug, Error)]
pub enum CppGenError {
    #[error("Failed to write file")]
    IoError(std::io::Error),
}

impl From<std::io::Error> for CppGenError {
    fn from(err: std::io::Error) -> Self {
        CppGenError::IoError(err)
    }
}
/// Errors that can occur while executing a command in another directory.
#[derive(Debug)]
pub enum CommandError {
    SetDirError(std::io::Error),
    CommandError(std::io::Error),
    ResetDirError(std::io::Error),
}
impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::SetDirError(e) => write!(f, "Set Directory Error: {}", e),
            CommandError::CommandError(e) => write!(f, "Command Error: {}", e),
            CommandError::ResetDirError(e) => write!(f, "Reset Directory Error: {}", e),
        }
    }
}

impl StdError for CommandError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            CommandError::SetDirError(e) => Some(e),
            CommandError::CommandError(e) => Some(e),
            CommandError::ResetDirError(e) => Some(e),
        }
    }
}

impl From<io::Error> for CommandError {
    fn from(error: io::Error) -> Self {
        CommandError::CommandError(error)
    }
}
