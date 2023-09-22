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
        let default_template = 
r#"
#include <emp-zk/emp-zk.h>

#include "emp-tool/emp-tool.h"

const int threads = 1;
const string circuit_file_location = macro_xstr(EMP_CIRCUIT_PATH) + string("bristol_format/");
int main(int argc, char ** argv) {
    int party, port;
    parse_party_and_port(argv, & party, & port);
    string filename = circuit_file_location + string("sha-256.txt");
    BoolIO < NetIO > * ios[threads];
    for (int i = 0; i < threads; ++i)
    ios[i] = new BoolIO < NetIO > (new NetIO(party == ALICE ? nullptr : "127.0.0.1", port + i), party == ALICE);
    setup_zk_bool < BoolIO < NetIO >> (ios, threads, party);

    auto bits_1 = convertEntryToBooleanArray("out.txt", 0);
    auto bits_2 = convertEntryToBooleanArray("out.txt", 1);
    auto bits_3 = convertEntryToBooleanArray("out.txt", 2);
    auto bits_4 = convertEntryToBooleanArray("out.txt", 3);
    auto bits_5 = convertEntryToBooleanArray("out.txt", 4);
    auto bits_6 = convertEntryToBooleanArray("out.txt", 5);
    auto bits_7 = convertEntryToBooleanArray("out.txt", 6);
    auto bits_8 = convertEntryToBooleanArray("out.txt", 7);

    bool * num_1 = new bool[256];
    bool * num_2 = new bool[256];
    bool * num_3 = new bool[256];
    bool * num_4 = new bool[256];
    bool * num_5 = new bool[256];
    bool * num_6 = new bool[256];
    bool * num_7 = new bool[256];
    bool * num_8 = new bool[256];

    memset(num_1, false, 256);
    memset(num_2, false, 256);
    memset(num_3, false, 256);
    memset(num_4, false, 256);
    memset(num_5, false, 256);
    memset(num_6, false, 256);
    memset(num_7, false, 256);
    memset(num_8, false, 256);

    for (size_t i = 0; i < 256; i++) {
    num_1[i] = bits_1[i];
    num_2[i] = bits_2[i];
    num_3[i] = bits_3[i];
    num_4[i] = bits_4[i];
    num_5[i] = bits_5[i];
    num_6[i] = bits_6[i];
    num_7[i] = bits_7[i];
    num_8[i] = bits_8[i];
    }
    bool ** array_leaves = new bool * [8];
    array_leaves[0] = num_1;
    array_leaves[1] = num_2;
    array_leaves[2] = num_3;
    array_leaves[3] = num_4;
    array_leaves[4] = num_5;
    array_leaves[5] = num_6;
    array_leaves[6] = num_7;
    array_leaves[7] = num_8;

    sort_leaves(array_leaves, 8, 256);

    MerkleTree tree(8, 4, array_leaves, filename);
    int * tree_path;
    tree.init_verify_path( & tree_path);

    auto leaf_bits = convertEntryToBooleanArray("out.txt", 0);
    bool * actual_leaf = new bool[256];
    memset(actual_leaf, false, 256);
    for (size_t q = 0; q < 256; q++) {
    actual_leaf[q] = leaf_bits[q];
    }

    tree.prove_in_tree(actual_leaf, tree_path);
    finalize_zk_bool < BoolIO < NetIO >> ();
    for (int i = 0; i < threads; ++i) {
    delete ios[i] -> io;
    delete ios[i];
    }
    return 0;
}"#
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
