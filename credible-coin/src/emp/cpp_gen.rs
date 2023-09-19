use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

/// Errors that can occur while generating C++ files.
#[derive(Debug)]
pub enum CppGenError {
    IoError(std::io::Error),
}

impl From<std::io::Error> for CppGenError {
    fn from(err: std::io::Error) -> Self {
        CppGenError::IoError(err)
    }
}
/// Errors that can occur while copying a file.
#[derive(Debug)]
pub enum FileError {
    IoError(std::io::Error),
    FileNotFound,
}

impl From<std::io::Error> for FileError {
    fn from(err: std::io::Error) -> Self {
        FileError::IoError(err)
    }
}

/// C++ file generator.
pub struct CppFileGenerator {
    directory: String,
    template: String,
}

impl CppFileGenerator {
    /// Creates a new generator with a default template.
    pub fn new(directory: &str) -> Self {
        let default_template = r#"
#include <iostream>

int main() {
    std::cout << "Hello, world from {{filename}}!" << std::endl;
    return 0;
}
"#
        .to_string();

        Self {
            directory: directory.to_string(),
            template: default_template,
        }
    }

    /// Sets a custom template for the generator. Use {{filename}} as a placeholder.
    pub fn with_template(mut self, template: &str) -> Self {
        self.template = template.to_string();
        self
    }

    /// Generates a C++ file based on the configuration.
    pub fn generate(&self, filename: &str) -> Result<(), CppGenError> {
        let path = Path::new(&self.directory).join(format!("{}.cpp", filename));
        let mut file = File::create(&path)?;

        let content = self.template.replace("{{filename}}", filename);
        file.write_all(content.as_bytes())?;

        Ok(())
    }
}
/// Copies the specified file to the given directory.
///
/// # Arguments
///
/// * `filename` - The full filename including its extension.
/// * `dest_dir` - The destination directory to which the file should be copied.
pub fn copy_to_directory(filename: &str, dest_dir: &str) -> Result<(), FileError> {
    let source_file_path = Path::new(filename);

    if !source_file_path.exists() {
        return Err(FileError::FileNotFound);
    }

    let destination_path = Path::new(dest_dir).join(source_file_path.file_name().unwrap());
    fs::copy(&source_file_path, destination_path)?;

    Ok(())
}
