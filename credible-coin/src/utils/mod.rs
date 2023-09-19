use std::env;
use std::ffi::OsString;
use std::fs::read_dir;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

pub mod address_generator;
pub mod csv_utils;
/// A helper trait to convert vector and slice types
/// to Vec<&[u8]> to be hashed by the sha crate
pub mod hashable;
pub mod merkle_utils;

/// Read the nth line from the file if it exists
pub fn nth_line_from_file(filename: &str, n: usize) -> io::Result<Option<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let line = reader.lines().nth(n);

    match line {
        Some(Ok(line_content)) => Ok(Some(line_content)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}
/// Get the project root (relative to closest Cargo.lock file)
/// ```rust
/// match project_root::get_project_root() {
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
