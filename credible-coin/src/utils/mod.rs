//! A module containing various utilities for csv parsing, file generation, and serialization
use std::env;
use std::fs::read_dir;
use std::io::{self};
use std::path::PathBuf;
use std::sync::Mutex;

/// A simple binary serializer
pub mod binary_serializer;
/// Helper functions for bitcoin
pub mod bitcoin_utils;
/// A set of csv helper functions
pub mod csv_utils;
/// A helper trait to convert vector and slice types
/// to Vec<&[u8]> to be hashed by the sha crate
pub mod hashable;
/// Helper functions to work with the merkle tree from `rs::merkle`
pub mod merkle_utils;
lazy_static! {
    static ref PROJECT_ROOT: Mutex<Option<String>> = Mutex::new(None);
}

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
    let mut root_cache = PROJECT_ROOT.lock().unwrap();

    if let Some(ref root) = *root_cache {
        return Ok(root.clone());
    }

    let path = env::current_dir()?;
    let mut path_ancestors = path.as_path().ancestors();

    while let Some(p) = path_ancestors.next() {
        let has_cargo = read_dir(p)?
            .filter_map(|entry| entry.ok())
            .any(|entry| entry.file_name() == "Cargo.lock");
        if has_cargo {
            let root = PathBuf::from(p).to_str().unwrap().to_owned();
            *root_cache = Some(root.clone());
            return Ok(root);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Ran out of places to find Cargo.lock",
    ))
}
