use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
