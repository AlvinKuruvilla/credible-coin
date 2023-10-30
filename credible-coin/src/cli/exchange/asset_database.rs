use std::{collections::HashSet, ffi::OsStr, path::Path, sync::Mutex};

use clap::Parser;
use csv::Writer;
use rs_merkle::{algorithms::Sha256, MerkleTree};

use crate::utils::{
    bitcoin_utils::generate_address,
    csv_utils::{make_value_vector, CSVRecord},
    merkle_utils::load_merkle_leaves_from_csv,
};

use super::shell::ExchangeShell;

pub(crate) fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .map(|s| s.trim())
}
/// Represents the CLI command for creating an exchange database.
///
/// This struct encapsulates the necessary details to generate an exchange database
/// from a publisher file. The generated exchange database will contain a certain number
/// of rows as specified by `row_count`.
///
/// # Fields
///
/// * `publisher_filename`: The path to the source CSV file containing the publisher data.
///
/// * `exchange_filename`: The path where the generated exchange CSV file will be saved.
///
/// * `row_count`: The number of rows to consider from the publisher file when generating
/// the exchange database.
///

#[derive(Parser, Debug)]
#[command(infer_subcommands = true)]
pub struct CreateCmd {
    publisher_filename: String,
    exchange_filename: String,
    row_count: usize,
}
/// Represents the CLI command for loading an exchange database.
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
    pub(crate) fn run(&self) {
        create_exchange_database(
            &self.publisher_filename,
            &self.exchange_filename,
            self.row_count,
        );
    }
}
impl LoadCmd {
    pub(crate) fn run(&self) {
        if !std::path::Path::new(&self.filename).exists() {
            panic!("Exchange file: {} not found", self.filename)
        }
        if get_extension_from_filename(&self.filename).unwrap() == "csv" {
            let merkle_leaves = load_merkle_leaves_from_csv(&self.filename);
            let coin_tree = load_exchange_db(merkle_leaves);
            // I think the clone is unavoidable, hopefully it doesn't bite us
            let mut exchange_shell = ExchangeShell::new(Some(coin_tree), self.filename.clone());
            match exchange_shell.start() {
                Ok(_) => {}
                Err(err) => log::error!("{}", err),
            }
        } else if get_extension_from_filename(&self.filename).unwrap() == "txt" {
            let mut exchange_shell = ExchangeShell::new(None, self.filename.clone());
            match exchange_shell.start() {
                Ok(_) => {}
                Err(err) => log::error!("{}", err),
            }
        } else {
            panic!(
                "Unrecognized extension: {} ",
                get_extension_from_filename(&self.filename).unwrap()
            )
        }
    }
}
/// Counts the number of rows in a given CSV file.
///
/// This function iterates over the entries of a CSV file and returns the total number of rows.
/// Note: The function counts both data rows and header rows. If the CSV file has a header, the
/// returned count will include it.
///
/// # Arguments
///
/// * `filepath` - The path to the CSV file.
///
/// # Returns
///
/// The total number of rows in the CSV file.
///
/// # Panics
///
/// This function will panic if the file does not exist or if any row in the CSV file cannot be
/// deserialized into a `CSVRecord`.
pub fn max_rows_in_csv(filepath: &str) -> usize {
    let mut rdr = csv::Reader::from_path(filepath).unwrap();
    let mut row_count: usize = 0;
    for result in rdr.deserialize() {
        let _: CSVRecord = result.unwrap();
        row_count += 1;
    }
    row_count
}
/// Creates an exchange database file.
///
/// This function first verifies the existence of the publisher file and the absence of an existing exchange file.
/// Then, it ensures that the desired row count is not exceeding the maximum available rows in the publisher CSV.
/// If all validations pass, it proceeds to create an exchange file, ensuring unique addresses and values.
///
/// # Arguments
///
/// * `publisher_filename` - The path to the CSV file used as a reference.
/// * `exchange_filename` - The desired path for the resulting exchange database file.
/// * `row_count` - The desired number of rows to be generated for the exchange file.
///
/// # Panics
///
/// * If the publisher file doesn't exist.
/// * If the exchange file already exists.
/// * If the provided row count exceeds the max rows in the publisher CSV.
///
pub fn create_exchange_database(
    publisher_filename: &str,
    exchange_filename: &str,
    row_count: usize,
) {
    if !std::path::Path::new(&publisher_filename).exists() {
        panic!("Publisher file: {} not found", publisher_filename)
    }
    if std::path::Path::new(&exchange_filename).exists() {
        panic!("Exchange file: {} already exists", exchange_filename)
    }
    let max_rows = max_rows_in_csv(publisher_filename);
    if row_count > max_rows {
        panic!(
            "Provided row count {} is greater than max row count of {} in publisher csv file",
            row_count, max_rows
        )
    }
    let mut selected_addresses: HashSet<String> = HashSet::new();
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

    // This should ensure that we do not have any repeated addresses making the file shorter
    while selected_addresses.len() != row_count {
        selected_addresses.insert(generate_address());
    }
    assert_eq!(selected_addresses.len(), row_count);
    let selected_values: Vec<i64> = make_value_vector(publisher_filename)[0..row_count].to_vec();
    assert_eq!(selected_addresses.len(), selected_values.len());
    std::fs::File::create(exchange_filename).unwrap();
    let mut writer: Writer<std::fs::File> = Writer::from_path(exchange_filename).unwrap();
    let _ = writer.write_record(["addresses", "value"]);
    for (index, address) in selected_addresses.iter().enumerate() {
        let _ = writer.write_record([address, &selected_values[index].to_string()]);
    }
    writer.flush().unwrap();
}

/// Loads an exchange database into a Merkle Tree.
///
/// Constructs a Merkle Tree with a Sha256 hash function using provided exchange coin leaves.
///
/// # Arguments
///
/// * `coin_leaves` - A vector of byte arrays representing the coin leaves for the exchange.
///
/// # Returns
///
/// A `MerkleTree<Sha256>` constructed from the provided coin leaves.
///
pub fn load_exchange_db(coin_leaves: Vec<[u8; 32]>) -> MerkleTree<Sha256> {
    MerkleTree::<Sha256>::from_leaves(&coin_leaves)
}
