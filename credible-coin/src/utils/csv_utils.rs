use std::fs::OpenOptions;

use csv::{Reader, Writer};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{errors::AddressPositionError, merkle_tree_entry::MerkleTreeEntry};

#[derive(Debug, Deserialize, Serialize)]
/// Represents a record from a CSV file.
///
/// This struct models the data structure for a record in a CSV file, primarily
/// focusing on an address and its associated value. The structure utilizes
/// serde's alias attribute to accommodate CSVs with slightly different column names.
///
/// # Fields
///
/// - `addresses`: The source address from the CSV. Can be found under the "source_address" column.
/// - `value`: The associated value for the given address. Can be located under either
///   the "delta" or "satoshi" columns in the CSV.
pub struct CSVRecord {
    #[serde(alias = "source_address")]
    addresses: String,
    #[serde(alias = "delta", alias = "satoshi")]
    value: i64,
}

fn find_matching_indices<T: PartialEq + ToString + Sync, U: PartialEq + ToString + Sync>(
    first_vector: &[T],
    val1: &T,
    second_vector: &[U],
    val2: &U,
) -> Result<usize, AddressPositionError> {
    assert_eq!(first_vector.len(), second_vector.len());
    first_vector
        .par_iter()
        .zip(second_vector.par_iter())
        .position_first(|(x, y)| x == val1 && y == val2)
        .ok_or_else(|| AddressPositionError::NoMatchingIndices {
            address: val1.to_string(),
            value: val2.to_string(),
        })
}
/// Given a filename as input return the value
/// column as a `Vec<i64>`
pub fn make_value_vector(filename: &str) -> Vec<i64> {
    let mut rdr = csv::Reader::from_path(filename).unwrap();
    let records: Vec<CSVRecord> = rdr
        .deserialize()
        .map(|result| result.expect("Error parsing CSV record"))
        .collect();
    records.par_iter().map(|record| record.value).collect()
}
/// Given a filename as input return the address column as a `Vec<String>`
/// NOTE 1: The current implementation forces the returned Vec to be a
/// `Vec<String>`. If you need the value column call the `make_value_vector`
/// function
/// NOTE 2: We cannot parallelize this function because doing par_bridge when
/// deserializing messes with element order guarantees vector gives
// TODO: Cache the vector?
pub fn make_address_vector(file_name: &str) -> Vec<String> {
    let mut rdr = csv::Reader::from_path(file_name).unwrap();

    let records: Vec<String> = rdr
        .deserialize::<CSVRecord>()
        .filter_map(|result| match result {
            Ok(record) => Some(record.addresses.clone()),
            Err(_) => unreachable!(),
        })
        .collect();

    records
}
/// Retrieve the address and value columns in the dataframe as vectors
pub fn addresses_and_values_as_vectors(file_name: &str) -> (Vec<String>, Vec<i64>) {
    let address_vec = make_address_vector(file_name);
    let value_vec = make_value_vector(file_name);
    (address_vec, value_vec)
}
/// Given a filename, a public address, and optional unique in that file, find its position within the address vector
pub fn get_address_position(
    filename: &str,
    public_address: String,
    value: Option<i64>,
) -> Result<usize, AddressPositionError> {
    let address_vec = make_address_vector(filename); // This function should properly handle errors and possibly return Result

    if let Some(val) = value {
        let values = make_value_vector(filename); // This function should properly handle errors and possibly return Result
        find_matching_indices(&address_vec, &public_address, &values, &val)
            .map_err(|_| AddressPositionError::NoMatchingIndexForValue(public_address, val))
    } else {
        address_vec
            .par_iter()
            .position_first(|r| r == &public_address)
            .ok_or_else(|| AddressPositionError::NoMatchingAddress(public_address))
    }
}
/// Update the value for the given address in a provided dataset file
/// with the provided value
/// This function works by updating the record in the old csv file, creating a new temporary one
/// and renaming it to the same name as the old file
pub fn update_csv_value(filename: &str, address: String, value: i64) {
    let mut rdr = Reader::from_path(filename).unwrap();
    let mut writer = Writer::from_path("temp.csv").unwrap();
    for result in rdr.deserialize() {
        let mut record: CSVRecord = result.unwrap();
        if record.addresses == address {
            record.value = value;
        }
        let _ = writer.serialize(record);
    }
    std::fs::remove_file(filename).unwrap();
    std::fs::rename("temp.csv", filename).unwrap();
}
/// Extracts exchange addresses and their associated values from a given CSV file.
///
/// This function reads the provided CSV file, extracts the `addresses` and `value`
/// columns, and returns them as separate vectors.
///
/// # Parameters
///
/// - `file_name`: The path to the CSV file.
///
/// # Returns
///
/// A tuple containing two vectors:
/// 1. A `Vec<String>` of extracted addresses.
/// 2. A `Vec<i64>` of the corresponding values for each address.
///
/// # Panics
///
/// This function will panic if:
/// - The provided file path is not found or there's an error in reading the file.
/// - There's a problem in deserializing the CSV records into the `CSVRecord` struct.
pub fn get_exchange_addresses_and_values_from_file(file_name: &str) -> (Vec<String>, Vec<i64>) {
    let mut rdr: Reader<std::fs::File> = csv::Reader::from_path(file_name).unwrap();
    let mut address_col = Vec::new();
    let mut val_col = Vec::new();
    for result in rdr.deserialize() {
        let record: CSVRecord = result.unwrap();
        address_col.push(record.addresses);
        val_col.push(record.value);
    }
    (address_col, val_col)
}
/// Given a file, and an address and value, write it as a record
/// to the end of the file
pub fn append_record(file: &str, address: String, value: u64) {
    let mut rdr: Reader<std::fs::File> = csv::Reader::from_path(file).unwrap();
    for result in rdr.deserialize() {
        let record: CSVRecord = result.unwrap();
        if record.addresses == address {
            log::error!("Record for address {}, already exists", address);
            return;
        }
    }
    let file_handle = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file)
        .unwrap();
    let mut writer = Writer::from_writer(file_handle);
    if writer.write_record(&[address, value.to_string()]).is_ok() {
    } else {
        log::error!("Failed to write record");
    }
    let _ = writer.flush();
}
/// Transforms paired vectors of strings and integers into a vector of `MerkleTreeEntry` structs.
///
/// This function takes a tuple of vectors containing strings and integers, and constructs
/// a new vector of `MerkleTreeEntry` structs, where each entry is initialized with an
/// element from both input vectors.
///
/// # Parameters
///
/// - `set`: A tuple containing:
///   1. A `Vec<String>` of addresses or any string identifiers.
///   2. A `Vec<i64>` of associated integer values.
///
/// # Returns
///
/// A `Vec<MerkleTreeEntry>`, where each entry is constructed from paired elements of
/// the input vectors.
///
/// # Panics
///
/// This function assumes that both vectors in the input tuple are of the same length.
/// If they are not, the function will panic due to the behavior of the `zip` function.
pub fn into_merkle_tree_entries(set: (Vec<String>, Vec<i64>)) -> Vec<MerkleTreeEntry> {
    let (vec1, vec2) = set;
    let mut ret = Vec::new();
    // Using the zip function to iterate over both vectors simultaneously
    for (item_from_vec1, item_from_vec2) in vec1.iter().zip(vec2.iter()) {
        ret.push(MerkleTreeEntry::new(
            item_from_vec1.clone(),
            *item_from_vec2,
        ));
    }
    ret
}
