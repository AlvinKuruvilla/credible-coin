use polars::prelude::*;
pub fn read_bitcoin_address_series() -> Series {
    let df = CsvReader::from_path("BigQuery Bitcoin Historical Data - outputs.csv")
        .unwrap()
        .finish()
        .unwrap();
    let addresses = df.column("addresses").unwrap().clone();
    return addresses;
}
