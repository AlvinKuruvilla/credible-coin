use std::{collections::HashSet, ffi::OsStr, path::Path, sync::Mutex};

use clap::Parser;
use csv::Writer;
use rs_merkle::{algorithms::Sha256, MerkleTree};

use crate::utils::{
    csv_utils::{make_value_vector, CSVRecord},
    file_generators::generate_address,
    merkle_utils::load_merkle_leaves_from_csv,
};

use super::shell::ExchangeShell;

pub(crate) fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .map(|s| s.trim())
}

#[derive(Parser, Debug)]
#[command(infer_subcommands = true)]
pub struct CreateCmd {
    publisher_filename: String,
    exchange_filename: String,
    row_count: usize,
}

#[derive(Parser, Debug)]
#[command(infer_subcommands = true)]
pub struct LoadCmd {
    filename: String,
}
impl CreateCmd {
    pub fn run(&self) {
        create_exchange_database(
            &self.publisher_filename,
            &self.exchange_filename,
            self.row_count,
        );
    }
}
impl LoadCmd {
    pub fn run(&self) {
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
pub fn max_rows_in_csv(filepath: &str) -> usize {
    let mut rdr = csv::Reader::from_path(filepath).unwrap();
    let mut row_count: usize = 0;
    for result in rdr.deserialize() {
        let _: CSVRecord = result.unwrap();
        row_count += 1;
    }
    row_count
}

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

// Loads a merkle tree from the coin leaves
pub fn load_exchange_db(coin_leaves: Vec<[u8; 32]>) -> MerkleTree<Sha256> {
    MerkleTree::<Sha256>::from_leaves(&coin_leaves)
}
