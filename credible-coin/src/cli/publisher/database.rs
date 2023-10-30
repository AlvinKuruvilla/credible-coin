use clap::Parser;
use csv::Writer;
use rs_merkle::{algorithms::Sha256, MerkleTree};
use std::path::Path;
use std::sync::Mutex;

use crate::cli::publisher::shell::PublisherShell;
use crate::utils::bitcoin_utils::generate_n_address_value_pairs;
use crate::utils::merkle_utils::load_merkle_leaves_from_csv;
/// Represents the CLI command for creating a publisher database of a specific size.
///
/// This struct encapsulates the details needed to generate a dataset with a given
/// number of rows and save it to a specified file. It primarily serves as a
/// command configuration for data-creation operations.
///
/// # Fields
///
/// * `out_filename`: The path where the generated data should be saved.
/// * `row_count`: The number of rows of data to generate.
///

#[derive(Parser, Debug)]
#[command(infer_subcommands = true)]
pub struct CreateCmd {
    out_filename: String,
    row_count: u32,
}
/// Represents the CLI command for loading a publisher database.
///
/// This struct encapsulates the necessary details to load data from a specified csv file.
///
/// # Fields
///
/// * `filename`: The path to the csv file from which the data should be loaded.

#[derive(Parser, Debug)]
#[command(infer_subcommands = true)]
pub struct LoadCmd {
    filename: String,
}
impl CreateCmd {
    // TODO: This needs to return an eyere::Result<()> at the end
    /// Create the db
    pub fn run(self) {
        // 1. Check that the out_file doesn't already exist and handle errors
        // 2. Create the new file
        create_db(&self.out_filename, self.row_count);
    }
}
impl LoadCmd {
    // TODO: This needs to return an eyere::Result<()> at the end
    /// Load the db
    pub fn run(self) {
        // 1. Check if the provided csv path exists and handle errors
        assert!(Path::new(&self.filename)
            .try_exists()
            .expect("Can't find the file"));
        // 2. Try to read as dataframe and handle errors
        // 3. Try to get the data from the addresses and values columns and handle errors
        // 4. Turn into merkle tree and handle errors
        let merkle_leaves = load_merkle_leaves_from_csv(&self.filename);
        let coin_tree = load_db(merkle_leaves.clone());
        let mut publisher_shell = PublisherShell::new(coin_tree, self.filename);
        let _ = publisher_shell.start();
    }
}
/// Creates csv file from random addresses and values
pub fn create_db(filename: &str, row_count: u32) {
    assert!(
        !Path::new(filename)
            .try_exists()
            .expect("file already exists"),
        "file already exists"
    );
    // NOTE: The only point of this guard is to protect against
    // potential race conditions if this function executes in parallel
    //
    // For example:
    //
    // If we are running benchmarks where we repeatedly create and delete
    // the generated file and the benchmarks are parallelized,
    // the file creation and deletion can race
    //
    // The guard is also implicitly dropped at the end of the function scope
    // so we don't need to explicitly call drop

    let mutex = Mutex::new(());
    let _guard = mutex.lock().unwrap();

    let (addresses, values) = generate_n_address_value_pairs(row_count);
    // Create the file but don't save the handle
    std::fs::File::create(filename).unwrap();
    let mut writer = Writer::from_path(filename).unwrap();
    assert_eq!(addresses.len(), values.len());
    writer.write_record(["addresses", "value"]).unwrap();

    for (index, address) in addresses.iter().enumerate() {
        writer
            .write_record([address, &values[index].to_string()])
            .unwrap();
    }
    writer.flush().unwrap();
}

/// Loads a merkle tree from the coin leaves
pub fn load_db(coin_leaves: Vec<[u8; 32]>) -> MerkleTree<Sha256> {
    MerkleTree::<Sha256>::from_leaves(&coin_leaves)
}
