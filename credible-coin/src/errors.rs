//! The various error types we use

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
    #[error("Unexpected: `{expected:?}` arguments found `{actual:?}` arguments")]
    /// An unexpected number of arguments was provided.
    UnexpectedNumberOfArguments {
        /// The expected number of arguments
        expected: usize,
        /// The actual number of arguments provided by the user
        actual: usize,
    },
    #[error("Empty argument provided at position {arg_position:?}")]
    /// An empty argument was provided at a specific position.
    EmptyArgument {
        /// The position of the empty argument in the argument list.
        arg_position: usize,
    },
}
/// Errors that can occur when trying to resolve the position of an address from a file
#[derive(Error, Debug)]
pub enum AddressPositionError {
    #[error("no matching index found for address: {0}, value: {1}")]
    /// No matching index for value
    NoMatchingIndexForValue(String, i64),
    #[error("no matching address found: {0}")]
    /// No matching address
    NoMatchingAddress(String),
    #[error("no matching indices found for {address} with value: {value}")]
    /// No matching indices
    NoMatchingIndices {
        /// The address
        address: String,
        /// The value
        value: String,
    },
}
/// Errors that can occur while copying a file.
#[derive(Debug, Error)]
pub enum FileError {
    #[error("Failed to write file")]
    /// I/O error
    IoError(std::io::Error),
    #[error("File not found")]
    /// File not found
    FileNotFound,
}

impl From<std::io::Error> for FileError {
    fn from(err: std::io::Error) -> Self {
        FileError::IoError(err)
    }
}
