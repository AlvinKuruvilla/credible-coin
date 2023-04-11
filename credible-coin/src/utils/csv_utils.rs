use polars::{
    prelude::{CsvReader, CsvWriter, DataFrame, SerReader, SerWriter},
    series::Series,
};
/// Read a test bitcoin dataset in the project root. For right now we assume
/// its the bigquery dataset but eventually the filename should be a parameter
pub fn read_bitcoin_address_dataframe(file_name: &str) -> DataFrame {
    //TODO: Remove unwrap and handle errors with match construct
    let df = CsvReader::from_path(file_name).unwrap().finish().unwrap();
    return df;
}
/// Given a dataframe as input return the specified
/// column as a series
pub fn get_dataset_column_by_name(file_name: &str, name: &str) -> Series {
    let df = read_bitcoin_address_dataframe(file_name);
    //TODO: Remove unwrap and handle errors with match construct
    return df.column(name).unwrap().clone();
}
/// Write a provided dataframe to a csv file of the provided filename in the
/// root of the project directory.
///
/// __NOTE: It is the caller's responsibility to preemptively check that the
/// filename they provided does not already exist before calling this
/// function__
pub fn write_csv(filename: &str, mut data: DataFrame) {
    let mut file = std::fs::File::create(filename).unwrap();
    CsvWriter::new(&mut file).finish(&mut data).unwrap();
}
/// Retrieve the address and value columns in the dataframe as vectors
pub fn addresses_and_values_as_vectors(file_name: &str) -> (Vec<String>, Vec<i64>) {
    let address_series = get_dataset_column_by_name(file_name, "addresses");
    let value_series = get_dataset_column_by_name(file_name, "value");
    // TODO: Remove unwrap()
    let value_vec = value_series.i64().unwrap().into_no_null_iter().collect();
    let address_vec = address_series
        .utf8()
        .unwrap()
        .into_no_null_iter()
        .map(|s| s.to_string())
        .collect();

    return (address_vec, value_vec);
}
/// Given a public address find its position within the address vector
pub fn get_address_position(public_address: String) -> usize {
    let address_series = get_dataset_column_by_name(
        "BigQuery Bitcoin Historical Data - outputs.csv",
        "addresses",
    );
    let address_vec: Vec<_> = address_series
        .utf8()
        .unwrap()
        .into_no_null_iter()
        .map(|s| s.to_string())
        .collect();
    // TODO: Remove unwrap()
    let index = address_vec
        .iter()
        .position(|r| r == &public_address)
        .unwrap();
    return index;
}
