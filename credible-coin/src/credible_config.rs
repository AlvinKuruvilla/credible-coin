//! Configuration for credible-coin.
//!
//! Required fields:
//! 1. circuit_file: Path to the SHA-256 Bristol format circuit file
//! 2. entries_file: Path to the entries file (address value pairs) for Merkle tree construction
use std::sync::RwLock;

use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct CredibleConfig {
    circuit_file: String,
    entries_file: String,
}
lazy_static! {
    static ref CONFIG: RwLock<Option<CredibleConfig>> = RwLock::new(None);
}
fn get_config() -> Result<CredibleConfig, ConfigError> {
    let config = Config::builder()
        .add_source(File::new("credible_config", FileFormat::Yaml))
        .build()
        .unwrap();
    let settings = config.try_deserialize::<CredibleConfig>()?;
    Ok(settings)
}

fn ensure_config() -> CredibleConfig {
    {
        let config_read = CONFIG.read().unwrap();
        if let Some(conf) = &*config_read {
            return conf.clone();
        }
    }

    let config = get_config().unwrap();
    let result = config.clone();

    let mut config_write = CONFIG.write().unwrap();
    *config_write = Some(config);

    result
}

/// Retrieves the path to the SHA-256 Bristol format circuit file.
pub fn get_circuit_file_path() -> String {
    ensure_config().circuit_file
}

/// Retrieves the path to the entries file for Merkle tree construction.
pub fn get_entries_file_path() -> String {
    ensure_config().entries_file
}
