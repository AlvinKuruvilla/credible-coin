use crate::credible_config::get_emp_root_path;
use crate::errors::CommandError;
use std::io::{self, Write};
use std::process::{Command, ExitStatus, Output, Stdio};
use std::sync::Mutex;
lazy_static! {
    static ref MAKE_LOCK: Mutex<()> = Mutex::new(());
    static ref BINARY_EXEC_LOCK: Mutex<()> = Mutex::new(());
}
/// Change the current directory to the specified one, execute the command with sudo, and revert back to the original directory.
///
/// # Arguments
///
/// * `dir` - The directory in which to execute the command.
/// * `command` - The command to execute.
/// * `args` - The arguments for the command.
///
/// # Return
/// The exit status of the command or an error if the command was unable to be executed.
pub fn sudo_execute(dir: &str, command: &str, args: &[&str]) -> Result<ExitStatus, CommandError> {
    // KEEP: Example of how we can hopefully ask for sudo permissions
    // while the program is running
    // https://users.rust-lang.org/t/how-to-execute-a-root-command-on-linux/50066/9
    //
    // assert!(::std::process::Command::new("sudo")
    //     .arg("/usr/bin/id")
    //     .status()
    //     .unwrap()
    //     .success());

    let exit_status = Command::new("sudo")
    .current_dir(dir) // Set the current directory directly on the Command
    .arg(command)
    .args(args)
    .stdout(Stdio::null()) // redirect everything from stdout to /dev/null
    .stderr(Stdio::null()) // redirect everything from stderr to /dev/null
    .status()?;
    if !exit_status.success() {
        eprintln!(
            "Command exited with non-zero status: {:?}",
            exit_status.code()
        );
    }

    Ok(exit_status)
}
/// Change the current directory to the specified one, execute the command with sudo, and revert back to the original directory.
///
/// # Arguments
///
/// * `dir` - The directory in which to execute the command.
/// * `command` - The command to execute.
/// * `args` - The arguments for the command.
///
/// # Return
/// The command output or an error if the command was unable to be executed.

pub fn sudo_execute_with_output(
    dir: &str,
    command: &str,
    args: &[&str],
) -> Result<Output, CommandError> {
    let output = Command::new("sudo")
    .current_dir(dir) // Set the current directory directly on the Command
    .arg(command)
    .args(args)
    .output()?;
    if !output.status.success() {
        eprintln!(
            "Command exited with non-zero status: {:?}",
            output.status.code()
        );
    }

    Ok(output)
}

/// Executes the `make install` command with multi-threading support.
///
/// This function leverages the number of available CPU cores to speed up the
/// compilation and installation process using the `-j` flag.
///
/// # Remarks
///
/// The function assumes that `ccache` is being used for caching and speeding up
/// recompilation. This is determined externally via environment variables.
/// Refer to [this StackOverflow answer](https://stackoverflow.com/a/37828605) for more details.
///
/// # Returns
///
/// Returns a `Result` wrapping the command's output. In case of any issues during execution,
/// it returns a `CommandError`.
///
pub fn execute_make_install() -> Result<ExitStatus, CommandError> {
    let _lock: std::sync::MutexGuard<'_, ()> = MAKE_LOCK.lock().unwrap();
    // Ccache should already be being used because I exported the environment
    // variable and saw the performance difference
    // See: https://stackoverflow.com/a/37828605
    let num_jobs = num_cpus::get().to_string();
    sudo_execute(&get_emp_root_path(), "make", &["install", "-j", &num_jobs])
}
/// Executes the `make install` command with multi-threading support.
///
/// This function leverages the number of available CPU cores to speed up the
/// compilation and installation process using the `-j` flag.
///
/// # Remarks
///
/// The function assumes that `ccache` is being used for caching and speeding up
/// recompilation. This is determined externally via environment variables.
/// Refer to [this StackOverflow answer](https://stackoverflow.com/a/37828605) for more details.
///
/// # Returns
///
/// Returns a `Result` wrapping the command's output. In case of any issues during execution,
/// it returns a `CommandError`.
///
pub fn execute_compiled_binary(binary_path: String) -> Result<Output, CommandError> {
    let _lock: std::sync::MutexGuard<'_, ()> = BINARY_EXEC_LOCK.lock().unwrap();
    sudo_execute_with_output(&get_emp_root_path(), "./run", &[&binary_path])
}
#[macro_export]
/// Handles the output of a command executed through `std::process::Command`.
///
/// This macro processes the `Result<Output, E>` from a command execution.
/// If the command ran successfully, it checks the exit status.
/// - If the command succeeded, it does nothing.
/// - If the command failed, it prints the command's standard error.
/// If there was an error while trying to run the command (e.g., command not found), it prints that error.
///
/// # Examples
///
/// ```no_run
/// use std::process::Command;
///
/// use credible_coin::handle_output;
///
/// let result = Command::new("ls").arg("-l").output();
/// handle_output!(result);
/// ```
///
macro_rules! handle_output {
    ($output:expr) => {
        match $output {
            std::result::Result::Ok(out) => {
                if out.status.success() {
                    // Print the standard output if the command succeeded
                    // println!("{}", String::from_utf8_lossy(&out.stdout));
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
#[macro_export]
/// Handles the status code of a command executed through `std::process::Command`.
///
/// This macro processes the `Result<ExitStatus, E>` from a command execution.
/// If the command ran successfully, it checks the exit status.
/// - If the command succeeded, it does nothing.
/// - If the command failed, it prints the command's standard error.
/// If there was an error while trying to run the command (e.g., command not found), it prints that error.
///
/// # Examples
///
/// ```no_run
/// use std::process::Command;
///
/// use credible_coin::handle_status;
///
/// let result = Command::new("ls").arg("-l").status();
/// handle_status!(result);
/// ```
///
macro_rules! handle_status {
    ($output:expr) => {
        match $output {
            std::result::Result::Ok(out) => {
                if out.success() {
                    // Print the standard output if the command succeeded
                    // println!("{}", String::from_utf8_lossy(&out.stdout));
                } else {
                    // Print the standard error if the command failed
                    eprintln!("Command Error Status: {}", &out.code().unwrap().to_string());
                }
            }
            Err(error) => {
                // Print the error if there's a problem running the command itself
                eprintln!("Execution Error: {:?}", error);
            }
        }
    };
}

/// Retrieves the membership string from the output of a command.
///
/// This function expects the command output to contain a line with the keyword "leaf".
/// It extracts this line and returns it as a `String`.
/// If no line with the keyword "leaf" is found, it returns an empty string.
///
/// # Parameters
///
/// - `output`: The output from a command execution.
///
/// # Returns
///
/// - `Ok(String)`: The line from the command output that contains the keyword "leaf", or an empty string if not found.
/// - `Err(CommandError)`: If there was an error executing the command.
///
/// # Examples
///
///
///
/// ```no_run
/// /// Assuming the command executed produces the following output:
/// // some random text
/// // leaf: example_data
/// // more random text
/// use std::process::{Command, Output};
/// use std::os::unix::process::ExitStatusExt;
/// use credible_coin::emp::executor::retrieve_membership_string;
/// use credible_coin::errors::CommandError;
/// fn mocked_command() -> Result<Output, CommandError> {
///     Ok(Output {
///         status: std::process::ExitStatus::from_raw(0),
///         stdout: b"some random text\nleaf: example_data\nmore random text".to_vec(),
///         stderr: vec![],
///     })
/// }
///
/// let output = mocked_command();
/// let membership = retrieve_membership_string(output).unwrap();
/// assert_eq!(membership, "leaf: example_data");
/// ```
///
/// Note: The example uses a mocked command execution function for demonstration purposes.

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
                None => Ok("".to_owned()),
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
/// We heavily rely on ccache to speed up "make install"
/// so we want the user to have it installed
pub fn is_ccache_installed() -> bool {
    Command::new("sh")
        .arg("-c")
        .arg("ccache --version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
