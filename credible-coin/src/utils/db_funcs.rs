use crate::utils::address_generator::*;
use polars::prelude::*;

pub fn createDB(){
	let mut datafr = generate_n_address_value_dataframe(5);
	let mut file = std::fs::File::create("test.csv").unwrap();
	CsvWriter::new(&mut file).finish(&mut datafr).unwrap();
}