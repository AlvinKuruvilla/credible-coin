use std::collections::HashMap;

use polars::series::Series;

/// A coin_map is a mapping of address to value pairs. It is safe to keep these mappings in plain-text
/// because this map is only used by the publisher. Internally, this just uses a Hashmap<String,i64>
#[derive(Default)]
pub struct CoinMap {
    pub inner: HashMap<String, i64>,
}
impl CoinMap {
    pub fn new() -> Self {
        return Self::default();
    }
    pub fn from_map(map: HashMap<String, i64>) -> Self {
        return Self { inner: map };
    }
    pub fn from_series(key_series: Series, value_series: Series) -> Self {
        let keys: Vec<String> = key_series
            .utf8()
            .unwrap()
            .into_no_null_iter()
            .map(|s| s.to_string())
            .collect();
        let values: Vec<i64> = value_series.i64().unwrap().into_no_null_iter().collect();
        let pairs: Vec<(String, i64)> = keys.into_iter().zip(values).collect();
        let map: HashMap<String, i64> = pairs.into_iter().collect();
        return Self { inner: map };
    }
    pub fn from_vectors(key_vector: Vec<String>, value_vector: Vec<i64>) -> Self {
        let pairs: Vec<(String, i64)> = key_vector.into_iter().zip(value_vector).collect();
        let map: HashMap<String, i64> = pairs.into_iter().collect();
        return Self { inner: map };
    }
    pub fn generate_address_value_map() -> Self {
        let (addresses, values) = crate::utils::csv_utils::addresses_and_values_as_vectors(
            "BigQuery Bitcoin Historical Data - outputs.csv",
        );
        return CoinMap::from_vectors(addresses, values);
    }
}
