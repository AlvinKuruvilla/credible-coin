use std::fs::OpenOptions;

use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CSVRecord {
    addresses: String,
    value: i64,
}

/// Given a filename as input return the value
/// column as a `Vec<i64>`
pub fn make_value_vector(filename: &str) -> Vec<i64> {
    let mut rdr = csv::Reader::from_path(filename).unwrap();
    let records: Vec<CSVRecord> = rdr
        .deserialize()
        .map(|result| result.expect("Error parsing CSV record"))
        .collect();
    return records.iter().map(|record| record.value.clone()).collect();
}
/// Given a filename as input return the specified
/// column as a `Vec<String>`
/// __The current implementation forces the returned Vec to be a `Vec<String>`. If
/// you need the value column call the `make_value_vector` function__
pub fn get_dataset_column_by_name(file_name: &str, name: &str) -> Vec<String> {
    //TODO: Remove unwrap and handle errors with match construct
    let mut rdr: Reader<std::fs::File> = csv::Reader::from_path(file_name).unwrap();
    let mut col = Vec::new();
    for result in rdr.deserialize() {
        let record: CSVRecord = result.unwrap();

        match name {
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
        let mut record: CSVRecord = result.unwrap();
        if record.addresses == address {
            record.value = value;
        }
        writer.serialize(record);
    }
    std::fs::remove_file(filename).unwrap();
    std::fs::rename("temp.csv", filename).unwrap();
    return;
}
pub fn get_exchange_addresses_and_values_from_file(file_name: &str) -> (Vec<String>, Vec<i64>) {
    let mut rdr: Reader<std::fs::File> = csv::Reader::from_path(file_name).unwrap();
    let mut address_col = Vec::new();
    let mut val_col = Vec::new();
    for result in rdr.deserialize() {
        let record: CSVRecord = result.unwrap();
        address_col.push(record.addresses);
        val_col.push(record.value);
    }
    return (address_col, val_col);
}
/// Given a file, and an address and value, write it as a record
/// to the end of the file
pub fn append_record(file: &str, adddress: String, value: i64) {
    let mut rdr: Reader<std::fs::File> = csv::Reader::from_path(file).unwrap();
    for result in rdr.deserialize() {
        let record: CSVRecord = result.unwrap();
        if record.addresses == adddress {
            log::error!("Record for address {}, already exists", adddress);
            return;
        }
    }
    let file_handle = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file)
        .unwrap();
    let mut writer = Writer::from_writer(file_handle);
    match writer.write_record(&[adddress, value.to_string()]) {
        Ok(_) => (),
        Err(_) => log::error!("Failed to write record"),
    }
    writer.flush();
}
