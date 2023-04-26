use indexmap::IndexMap;

/// A CoinMap is a mapping of address to value pairs. It is safe to keep these mappings in plain-text
/// because this map is only used by the publisher. Internally, this just uses a IndexMap<String,i64>
#[derive(Default)]
pub struct CoinMap {
    /// The `inner` type _must_ be an `IndexMap` so that insertion order can be maintained.
    /// This ensures that if a Merkle Tree is made from the map, we shouldn't get
    /// proof verification crashes from misordered keys
    pub inner: IndexMap<String, i64>,
}
impl CoinMap {
    pub fn new() -> Self {
        return Self::default();
    }
    pub fn from_map(map: IndexMap<String, i64>) -> Self {
        return Self { inner: map };
    }
    pub fn from_vectors(key_vector: Vec<String>, value_vector: Vec<i64>) -> Self {
        assert_eq!(key_vector.len(), value_vector.len());
        let mut map = IndexMap::new();
        for (pos, element) in key_vector.iter().enumerate() {
            // NOTE: If the address occurs multiple times, only the value will be updated
            // in the map so the map will have fewer keys compared to rows in the csv file
            // causing issues
            map.insert(element.to_owned(), value_vector[pos]);
        }
        return Self { inner: map };
    }
    pub fn generate_address_value_map(filename: &str) -> Self {
        let (addresses, values) =
            crate::utils::csv_utils::addresses_and_values_as_vectors(filename);
        println!("Address Length: {:?}", addresses.len());
        println!("Values Length: {:?}", values.len());
        return CoinMap::from_vectors(addresses, values);
    }

    pub fn replace(&mut self, address_key: String, new_val: i64) {
        assert!(self.inner.contains_key(&address_key));
        self.inner
            .entry(address_key)
            .and_modify(|old_value| *old_value = new_val);
    }
    pub fn dbg_print(&self) {
        for (key, value) in &self.inner {
            println!("{}: {}", key, value);
        }
    }
}
mod tests {
    use rs_merkle::{algorithms::Sha256, Hasher};

    use crate::coin::Coin;

    use super::CoinMap;

    #[test]
    pub fn byte_hash_changes_after_value_update() {
        let mut cm =
            CoinMap::generate_address_value_map("BigQuery Bitcoin Historical Data - outputs.csv");
        let old_value = cm
            .inner
            .get("bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k")
            .unwrap();
        assert_eq!(old_value.to_owned(), 22222);
        let old_coin = Coin::new(
            "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k".to_string(),
            *old_value,
        );
        let old_bytes = old_coin.serialize_coin();
        let old_hash = Coin::hash_bytes(old_bytes);

        cm.replace(
            "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k".to_string(),
            12345,
        );
        let retrieved_value = cm
            .inner
            .get("bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k")
            .unwrap();
        assert_eq!(retrieved_value.to_owned(), 12345);
        let coin = Coin::new(
            "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k".to_string(),
            *retrieved_value,
        );
        assert_ne!(coin.serialize_coin(), old_coin.serialize_coin());
        let bytes = coin.serialize_coin();
        let new_hash = Coin::hash_bytes(bytes);
        assert_ne!(old_hash, new_hash);
    }
    #[test]
    pub fn bytes_equality() {
        let mut address: String = "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k".to_owned();
        // &address.push_str(&22222.to_string());
        // assert_eq!(address, "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k22222".to_owned());
        address.push_str(&22222.to_string());
        let first = Sha256::hash(&bincode::serialize(&address).unwrap());

        let mut new_address: String = "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k".to_owned();
        new_address.push_str(&12345.to_string());
        let second = Sha256::hash(&bincode::serialize(&new_address).unwrap());

        assert_ne!(first, second);
    }
}
