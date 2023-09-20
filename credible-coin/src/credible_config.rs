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
pub fn get_emp_copy_path() -> String {
    let config = get_config().unwrap();
    config.emp_path
}
pub fn get_emp_root_path() -> String {
    let config = get_config().unwrap();
    config.emp_root_path
}
