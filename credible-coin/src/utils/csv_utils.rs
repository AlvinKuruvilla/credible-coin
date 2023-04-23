use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PublisherCSVRecord {
    transaction_hash: String,
    block_hash: String,
    block_number: String,
    block_timestamp: String,
    index: u32,
    script_asm: String,
    script_hex: String,
    required_signatures: String,
    hash_type: String,
    addresses: String,
    value: i64,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeCSVRecord {
    addresses: String,
    value: i64,
}
/// Given a filename as input return the value
/// column as a Vec<i64>
pub fn make_value_vector(filename: &str) -> Vec<i64> {
    let mut rdr = csv::Reader::from_path(filename).unwrap();
    let mut col = Vec::new();
    for result in rdr.deserialize() {
        let record: PublisherCSVRecord = result.unwrap();
        col.push(record.value)
    }
    return col;
}
/// Given a filename as input return the index
/// column as a Vec<u32>
pub fn make_index_vector(filename: &str) -> Vec<u32> {
    let mut rdr = csv::Reader::from_path(filename).unwrap();
    let mut col = Vec::new();
    for result in rdr.deserialize() {
        let record: PublisherCSVRecord = result.unwrap();
        col.push(record.index)
    }
    return col;
}
/// Given a filename as input return the specified
/// column as a Vec<String>
/// <mark> The current implementation forces the returned Vec to be a `Vec<String>. If
/// you need the index or value columns call the `make_index_vector` or `make_value_vector`
/// respectively </mark>
pub fn get_dataset_column_by_name(file_name: &str, name: &str) -> Vec<String> {
    //TODO: Remove unwrap and handle errors with match construct
    // FIXME: Factor this better
    let mut rdr: Reader<std::fs::File> = csv::Reader::from_path(file_name).unwrap();
    let mut col = Vec::new();
    for result in rdr.deserialize() {
        let record: PublisherCSVRecord = result.unwrap();

        match name {
            "transaction_hash" => col.push(record.transaction_hash),
            "block_hash" => col.push(record.block_hash),
            "block_number" => col.push(record.block_number),
            "block_timestamp" => col.push(record.block_timestamp),
            "script_asm" => col.push(record.script_asm),
            "script_hex" => col.push(record.script_hex),
            "hash_type" => col.push(record.hash_type),
            "addresses" => col.push(record.addresses),
            "value" | "index" =>  panic!("If you want to get the index or value column call the make_index_vector or make_value_vector function respectively"),
            _ => panic!("Unrecognized column name: {:?}", name)}
    }
    return col;
}
/// Retrieve the address and value columns in the dataframe as vectors
pub fn addresses_and_values_as_vectors(file_name: &str) -> (Vec<String>, Vec<i64>) {
    let address_vec = get_dataset_column_by_name(file_name, "addresses");
    let value_vec = make_value_vector(file_name);
    return (address_vec, value_vec);
}
/// Given a filename, and a public address in that file, find its position within the address vector
pub fn get_address_position(filename: &str, public_address: String) -> usize {
    let address_vec = get_dataset_column_by_name(filename, "addresses");
    // TODO: Remove unwrap()
    let index = address_vec
        .iter()
        .position(|r| r == &public_address)
        .unwrap();
    return index;
}
/// Update the value for the given address in a provided dataset file
/// with the provided value
/// This function works by updating the record in the old csv file, creating a new temporary one
/// and renaming it to the same name as the old file
pub fn update_csv_value(filename: &str, address: String, value: i64) {
    let mut rdr = Reader::from_path(filename).unwrap();
    let mut writer = Writer::from_path("temp.csv").unwrap();
    for result in rdr.deserialize() {
        let mut record: PublisherCSVRecord = result.unwrap();
        if record.addresses == address {
            record.value = value;
        }
        writer.serialize(record);
    }
    std::fs::remove_file(filename).unwrap();
    std::fs::rename("temp.csv", filename).unwrap();
    return;
}
// TODO: We should find a bettter way to write this, we shouldn't need to write functions for each recod
// type when we change the format of how we parse file. There are 2 options:
// 1. We should figure out a way to make the RecordType generic so we call one function
// and it would know how to parse the file
// 2. Remove the exchange fields from PublisherCSVRecord
pub fn get_exchange_addresses_and_values_from_file(file_name: &str) -> (Vec<String>, Vec<i64>) {
    let mut rdr: Reader<std::fs::File> = csv::Reader::from_path(file_name).unwrap();
    let mut address_col = Vec::new();
    let mut val_col = Vec::new();
    for result in rdr.deserialize() {
        let record: ExchangeCSVRecord = result.unwrap();
        address_col.push(record.addresses);
        val_col.push(record.value);
    }
    return (address_col, val_col);
}
