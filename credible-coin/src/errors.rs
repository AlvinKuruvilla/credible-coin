//! The various error types we use

use serde::ser::StdError;
use std::fmt;
use std::io;
use thiserror::Error;
#[derive(Error, Debug)]
/// Errors handling failures to connect to the Redis instance
pub enum DBConnectorError {
    #[error("Redis error: {0}")]
    /// A specialization error for redis::RedisError representing a Connection Error
    RedisConnectorError(#[from] redis::RedisError),
}
#[derive(Error, Debug)]
/// Errors that can occur while processing command-line interface (CLI) inputs.
pub enum CliError {
    /// An unexpected number of arguments was provided.
    ///
    /// This error is triggered when the number of provided arguments does not match
    /// the expected number.
    ///
    /// - `expected`: The expected number of arguments.
    /// - `actual`: The actual number of arguments provided by the user.
    #[error("Unexpected: `{expected:?}` arguments found `{actual:?}` arguments")]
    UnexpectedNumberOfArguments {
        /// The expected number of arguments
        expected: usize,
        /// The actual number of arguments provided by the user
        actual: usize,
    },

    /// An empty argument was provided at a specific position.
    ///
    /// This error indicates that one of the expected arguments was provided as empty.
    ///
    /// - `arg_position`: The position of the empty argument in the argument list.
    #[error("Empty argument provided at position {arg_position:?}")]
    EmptyArgument {
        /// The position of the empty argument in the argument list.
        arg_position: usize,
    },
}
/// Errors that can occur when trying to resolve the position of an address from a file
#[derive(Error, Debug)]
pub enum AddressPositionError {
    /// Indicates that no index was found for the given address and value combination.
    ///
    /// - `String`: The address for which the index was not found.
    /// - `i64`: The value for which the index was not found.
    #[error("no matching index found for address: {0}, value: {1}")]
    NoMatchingIndexForValue(String, i64),

    /// Indicates that no address was found that matches the provided address.
    ///
    /// - `String`: The address which was not found.
    #[error("no matching address found: {0}")]
    NoMatchingAddress(String),

    /// Indicates that no indices were found for the given address with a specific value.
    #[error("no matching indices found for {address} with value: {value}")]
    NoMatchingIndices {
        /// The address for which the indices were not found.
        address: String,
        /// The specific value for which the indices were not found
        value: String,
    },
}
/// Errors that can occur while copying a file.
#[derive(Debug, Error)]
pub enum FileError {
    /// Represents I/O errors encountered during file operations.
    ///
    /// Contains the underlying `std::io::Error` for detailed diagnostics.
    #[error("Failed to write file")]
    IoError(std::io::Error),

    /// Indicates that the desired file was not found.
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
    /// Represents I/O errors encountered during file operations in the code generation process.
    ///
    /// Contains the underlying `std::io::Error` for detailed diagnostics.
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
    /// Represents errors encountered when attempting to set the directory for a command.
    ///
    /// Contains the underlying `std::io::Error` for detailed diagnostics.
    SetDirError(std::io::Error),

    /// Represents errors encountered during the execution of a command.
    ///
    /// Contains the underlying `std::io::Error` for detailed diagnostics.
    CommandError(std::io::Error),

    /// Represents errors encountered when attempting to reset to the original directory after a command.
    ///
    /// Contains the underlying `std::io::Error` for detailed diagnostics.
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
