use std::collections::HashMap;
use std::path::Path;

use tokio::fs;
use tokio::sync::{Mutex, MutexGuard};

use crate::errors::CppGenError;

use super::template_engine::TemplateEngine;
lazy_static! {
    static ref COPY_LOCK: Mutex<()> = Mutex::new(());
}
/// A utility struct for generating C++ files.
///
/// It uses a given template and outputs the generated files into a specified directory.
///
/// # Attributes
///
/// * `directory`: The output directory where the generated C++ files will be stored.
/// * `template`: A template string used for generating the C++ file.
#[derive(Debug)]
pub struct CppFileGenerator {
    directory: String,
    template: String,
}

impl CppFileGenerator {
    /// Creates a new generator with a default template.
    pub fn new(directory: &str, substitution_map: HashMap<String, String>) -> Self {
        let default_template = r#"
#include <emp-zk/emp-zk.h>
#include <iostream>
#include <emp-tool/emp-tool.h>
using namespace emp;
using namespace std;

int port, party;
const int threads = 1;

const string circuit_file_location = macro_xstr(EMP_CIRCUIT_PATH) + string("bristol_format/");

int main(int argc, char **argv)
{
    int party, port;
    parse_party_and_port(argv, &party, &port);
    string filename = circuit_file_location + string("sha-256.txt");
    BoolIO<NetIO> *ios[threads];
    for (int i = 0; i < threads; ++i)
        ios[i] = new BoolIO<NetIO>(new NetIO(party == ALICE ? nullptr : "127.0.0.1", port + i), party == ALICE);
    setup_zk_bool<BoolIO<NetIO>>(ios, threads, party);

    bool **array_leaves = new bool *[8];
    dynamic_leaf_array_init(8, array_leaves);

    sort_leaves(array_leaves, 8, 256);

    MerkleTree tree(8, 4, array_leaves, filename);

    int *tree_path;
    tree.init_verify_path(&tree_path);

    auto leaf_bits = convertEntryToBooleanArray("out.txt", <<actual_leaf_index>>);
    bool *actual_leaf = new bool[256];

    memset(actual_leaf, false, 256);
    for (size_t q = 0; q < 256; q++)
    {
        actual_leaf[q] = leaf_bits[q];
    }
    tree.prove_in_tree(actual_leaf, tree_path);

    finalize_zk_bool<BoolIO<NetIO>>();
    for (int i = 0; i < threads; ++i)
    {
        delete ios[i]->io;
        delete ios[i];
    }

    return 0;
}
        "#;
        let engine = TemplateEngine::new();
        let filled_template = engine.render(default_template, substitution_map);
        Self {
            directory: directory.to_string(),
            template: filled_template,
        }
    }

    /// Sets a custom template for the generator. Use {{filename}} as a placeholder.
    pub fn with_template(mut self, template: &str) -> Self {
        self.template = template.to_string();
        self
    }

    /// Generates a C++ file based on the configuration.
    pub fn generate(&self, filename: &str) -> Result<(), CppGenError> {
        TemplateEngine::write_to_file(&self.template, filename, self.directory.clone())?;
        Ok(())
    }
}
/// Copies the specified file to the given directory.
///
/// # Arguments
///
/// * `filename` - The full filename including its extension.
/// * `dest_dir` - The destination directory to which the file should be copied.
//TODO: Retest performance on a fresh boot
pub async fn copy_to_directory(filename: &str, dest_dir: &str) -> std::io::Result<()> {
    let _lock: MutexGuard<'_, ()> = COPY_LOCK.lock().await;
    let source_file_path = Path::new(filename);
    let destination_path = Path::new(dest_dir).join(source_file_path.file_name().ok_or(
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid filename"),
    )?);

    fs::copy(&source_file_path, &destination_path).await?;

    Ok(())
}
