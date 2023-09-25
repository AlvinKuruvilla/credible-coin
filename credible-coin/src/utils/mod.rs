use std::env;
use std::ffi::OsString;
use std::fs::read_dir;
use std::io::ErrorKind;
use std::io::{self};
use std::path::PathBuf;

pub mod csv_utils;
pub mod file_generators;
/// A helper trait to convert vector and slice types
/// to Vec<&[u8]> to be hashed by the sha crate
pub mod hashable;
pub mod merkle_utils;

/// Get the project root (relative to closest Cargo.lock file)
/// ```rust
/// use credible_coin::utils::get_project_root;
/// match get_project_root() {
///     Ok(p) => println!("Current project root is {:?}", p),
///     Err(e) => println!("Error obtaining project root {:?}", e)
/// };
/// ```
// adapted from https://docs.rs/project-root/latest/project_root/fn.get_project_root.html
pub fn get_project_root() -> io::Result<String> {
    let path = env::current_dir()?;
    let mut path_ancestors = path.as_path().ancestors();

    while let Some(p) = path_ancestors.next() {
        let has_cargo = read_dir(p)?
            .into_iter()
            .any(|p| p.unwrap().file_name() == OsString::from("Cargo.lock"));
        if has_cargo {
            return Ok(PathBuf::from(p).to_str().unwrap().to_owned());
        }
    }
    Err(io::Error::new(
        ErrorKind::NotFound,
        "Ran out of places to find Cargo.toml",
    ))
}
