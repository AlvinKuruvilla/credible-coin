use polars::{
    prelude::{CsvReader, CsvWriter, DataFrame, SerReader, SerWriter},
    series::Series,
};
/// Read a test bitcoin dataset in the project root. For right now we assume
/// its the bigquery dataset but eventually the filename should be a parameter
pub fn read_bitcoin_address_dataframe() -> DataFrame {
    //TODO: Remove unwrap and handle errors with match construct
    let df = CsvReader::from_path("BigQuery Bitcoin Historical Data - outputs.csv")
        .unwrap()
        .finish()
        .unwrap();
    return df;
}
/// Given a dataframe as input return the specified
/// column as a series
pub fn get_dataset_column_by_name(name: &str) -> Series {
    let df = read_bitcoin_address_dataframe();
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
