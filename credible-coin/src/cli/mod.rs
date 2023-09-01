//! This module is responsible for creating the CLI and shell functionality of each component of our system.
//! The `bin/` folder then has an associated binary program for each of the components
//!
//! We have 4 executable program components  
//! 1. Publisher: The publisher acts like a pseudo cryptocurrency exchange responsible for pulling and
//! modifying address and value data from the blockchain. From there it can take this data and
//! load it into a Merkle tree for the exchange to use later to generate proofs
//! 2. Exchange: The exchange represents the company that a customer would store their assets on.
//! The exchange's main job is to communicate with the verifier component to generate solvency
//! proofs, and can perform various functions to that end. (See the module docs for more details)
//! 3. Verifier: Similar to the Exchange, the Verifier's main function is to send
//! solvency requests and manage the generated proofs accordingly. (See the module docs for more details)
//! 4. Customer: The customer's job is to take the generated proofs and run its own checks on them
//! utilizing our CLI/shell to make a more informed decision about the safety and security of
//! their assets  (See the module docs for more details)
use anyhow::{anyhow, ensure, Result};
use thiserror::Error;

pub mod exchange;
pub mod publisher;
/// ``ArgsList`` abstracts away the responsibility of input sanitization away from the caller and exposes matchable errors instead.
/// ``AgsList`` (on creation):
///
/// 1. Checks the number of arguments passed in (aside from the command itself)
/// 2. Checks for empty strings.
/// In any error case we should return a matchable error type, so the shell can do error handling
pub struct ArgsList {
    #[allow(dead_code)]
    args: Vec<String>,
}
#[derive(Error, Debug)]
pub enum CliError {
    #[error("Unexpected: `{expected:?}` arguments found `{actual:?}` arguments")]
    UnexpectedNumberOfArguments { expected: usize, actual: usize },
    #[error("Empty argument provided at position {arg_position:?}")]
    EmptyArgument { arg_position: usize },
}

fn check_empty_argument(args: &[String]) -> Result<()> {
    if let Some(index) = args.iter().position(|arg| arg.is_empty()) {
        return Err(anyhow!(CliError::EmptyArgument {
            arg_position: index
        }));
    }

    Ok(())
}
impl ArgsList {
    pub fn new(args_list: Vec<String>, arg_count: usize) -> Result<Self> {
        // We can also do some regex matching possibly to see if an argument is a file path with a valid extension
        ensure!(
            args_list.len() == arg_count,
            CliError::UnexpectedNumberOfArguments {
                expected: arg_count,
                actual: args_list.len(),
            }
        );
        check_empty_argument(&args_list)?;
        Ok(Self { args: args_list })
    }
}
fn convert_to_string_vec(elements: Vec<&str>) -> Vec<String> {
    return elements.iter().map(|&s| s.to_owned()).collect();
}

pub mod arg_sanitizer {
    //! Contains the `sanitize_args` macro
    use crate::cli::{ArgsList, CliError};
    macro_rules! sanitize_args {
        ($args:expr, $arg_count:expr, $empty_error:expr) => {
            match ArgsList::new($args[1..].to_vec(), $arg_count) {
                Ok(_) => {}
                Err(err) => match err.downcast_ref::<CliError>() {
                    Some(cli_error) => {
                        match cli_error {
                            CliError::UnexpectedNumberOfArguments {
                                expected,
                                actual,
                            } => {
                                // Handle unexpected number of arguments error
                                log::error!(
                                    "Error: Unexpected number of arguments. Expected: {}, Actual: {}",
                                    expected, actual
                                );
                                continue;
                            }
                            CliError::EmptyArgument { arg_position } => {
                                // Handle empty argument error with custom error message
                                log::error!(
                                    "Error: {} at position {}",
                                    $empty_error,
                                    arg_position + 1
                                );
                                continue;
                            }
                        }
                    }
                    None => log::error!("Unknown error: {}", err),
                },
            };
        };
    }
    pub(crate) use sanitize_args;
}
