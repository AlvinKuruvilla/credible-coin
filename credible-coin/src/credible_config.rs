//! A type holding all of our configuration properties. Namely:
//! 1. emp_path: The path to the emp project test directory where we put generated c++ files for membership proofs
//! 2. emp_root_path: The path to the emp project root directory
use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
struct CredibleConfig {
    emp_path: String,
    emp_root_path: String,
}
fn get_config() -> Result<CredibleConfig, ConfigError> {
    let config = Config::builder()
        .add_source(File::new("credible_config", FileFormat::Yaml))
        .build()
        .unwrap();
    let settings = config.try_deserialize::<CredibleConfig>()?;
    Ok(settings)
}
/// Retrieves the path for the EMP copy from the configuration.
///
/// This function fetches the configuration using the `get_config` function and returns
/// the `emp_path` from the configuration.
///
/// # Returns
///
/// A `String` containing the EMP copy path.
///
/// # Panics
///
/// This function will panic if it fails to fetch the configuration.
pub fn get_emp_copy_path() -> String {
    let config = get_config().unwrap();
    config.emp_path
}
/// Retrieves the root path for EMP from the configuration.
///
/// This function fetches the configuration using the `get_config` function and returns
/// the `emp_root_path` from the configuration.
///
/// # Returns
///
/// A `String` containing the EMP root path.
///
/// # Panics
///
/// This function will panic if it fails to fetch the configuration.

pub fn get_emp_root_path() -> String {
    let config = get_config().unwrap();
    config.emp_root_path
}
