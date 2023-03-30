use polars::prelude::*;
pub fn read_bitcoin_address_dataframe() -> DataFrame {
    let df = CsvReader::from_path("BigQuery Bitcoin Historical Data - outputs.csv")
        .unwrap()
        .finish()
        .unwrap();
    return df;
}
pub fn get_dataset_column_by_name(name: &str) -> Series {
    let df = read_bitcoin_address_dataframe();
    return df.column(name).unwrap().clone();
}
