use crate::credible_config::get_emp_root_path;
use crate::errors::CommandError;
use std::env;
use std::io::{self, Write};
use std::process::{Command, Output};

/// Change the current directory to the specified one, execute the command with sudo, and revert back to the original directory.
///
/// # Arguments
///
/// * `dir` - The directory in which to execute the command.
/// * `command` - The command to execute.
/// * `args` - The arguments for the command.
///
/// # Return
/// out: the output of the command

pub fn sudo_execute(dir: &str, command: &str, args: &[&str]) -> Result<Output, CommandError> {
    // KEEP: Example of how we can hopefully ask for sudo permissions
    // while the program is running
    // https://users.rust-lang.org/t/how-to-execute-a-root-command-on-linux/50066/9
    //
    // TODO: Check if this works in the shell environment as well
    // assert!(::std::process::Command::new("sudo")
    //     .arg("/usr/bin/id")
    //     .status()
    //     .unwrap()
    //     .success());

    // Store the current directory.
    let current_dir = env::current_dir().map_err(CommandError::SetDirError)?;

    // Change to the desired directory.
    env::set_current_dir(dir).map_err(CommandError::SetDirError)?;

    // Execute the command with sudo.
    let out = Command::new("sudo")
        .arg(command)
        .args(args)
        .output()
        .map_err(CommandError::CommandError)?;

    if !out.status.success() {
        eprintln!(
            "Command exited with non-zero status: {:?}",
            out.status.code()
        );
    }

    // Revert back to the original directory.
    env::set_current_dir(current_dir).map_err(CommandError::ResetDirError)?;

    Ok(out)
}
/// Change the current directory to the specified one, execute the command, and revert back to the original directory.
///
/// # Arguments
///
/// * `dir` - The directory in which to execute the command.
/// * `command` - The command to execute.
/// * `args` - The arguments for the command.
///
/// # Return
/// out: the output of the command

pub fn execute(dir: &str, command: &str, args: &[&str]) -> Result<(), CommandError> {
    // Store the current directory.
    let current_dir = env::current_dir().map_err(CommandError::SetDirError)?;

    // Change to the desired directory.
    env::set_current_dir(dir).map_err(CommandError::SetDirError)?;

    // Execute the command.
    let status = Command::new(command)
        .args(args)
        .status()
        .map_err(CommandError::CommandError)?;

    if !status.success() {
        eprintln!("Command exited with non-zero status: {:?}", status.code());
    }

    // Revert back to the original directory.
    env::set_current_dir(current_dir).map_err(CommandError::ResetDirError)?;

    Ok(())
}
pub fn execute_make_install() -> Result<Output, CommandError> {
    sudo_execute(&get_emp_root_path(), "make", &["install"])
}
pub fn execute_compiled_binary(binary_path: String) -> Result<Output, CommandError> {
    sudo_execute(&get_emp_root_path(), "./run", &[&binary_path])
}
#[macro_export]
macro_rules! handle_output {
    ($output:expr) => {
        match $output {
            std::result::Result::Ok(out) => {
                if out.status.success() {
                    // Print the standard output if the command succeeded
                    println!("{}", String::from_utf8_lossy(&out.stdout));
                } else {
                    // Print the standard error if the command failed
                    eprintln!("Command Error: {}", String::from_utf8_lossy(&out.stderr));
                }
            }
            Err(error) => {
                // Print the error if there's a problem running the command itself
                eprintln!("Execution Error: {:?}", error);
            }
        }
    };
}
pub fn retrieve_membership_string(
    output: Result<Output, CommandError>,
) -> Result<String, CommandError> {
    match output {
        Ok(success_output) => {
            // stdout is assumed to be of type Vec<u8>.
            let stdout = String::from_utf8_lossy(&success_output.stdout);
            let membership_string = match stdout
                .to_string()
                .lines()
                .filter(|line| line.contains("leaf"))
                .next()
            {
                Some(line) => Ok(line.to_string()), // Return the line if "leaf" is found.
                None => unimplemented!(),
            };
            membership_string
        }
        Err(err) => {
            // Write the error to stderr.
            // Adjust this part according to the actual definition of CommandError.
            let _ = writeln!(io::stderr(), "Command Error: {:?}", err);
            Err(err)
        }
    }
}
