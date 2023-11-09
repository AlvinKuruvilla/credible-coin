use indexmap::IndexMap;

/// A ``EntryMap`` is a mapping of address to value pairs. It is safe to keep these mappings in plain-text
/// because this map is only used by the publisher. Internally, this just uses a IndexMap<String,i64>
#[derive(Default, Debug)]
pub struct EntryMap {
    /// The `inner` type _must_ be an `IndexMap` so that insertion order can be maintained.
    /// This ensures that if a Merkle Tree is made from the map, we shouldn't get
    /// proof verification crashes from unordered keys
    pub inner: IndexMap<String, i64>,
}
impl EntryMap {
    /// Creates a new, empty `EntryMap`.
    ///
    /// # Returns
    ///
    /// Returns a default-initialized `EntryMap`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use credible_coin::cli::publisher::entry_map::EntryMap;
    /// let entry_map = EntryMap::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an `EntryMap` from an existing `IndexMap`.
    ///
    /// # Arguments
    ///
    /// * `map` - An `IndexMap` from which to initialize the `EntryMap`.
    ///
    /// # Returns
    ///
    /// Returns an `EntryMap` containing the data from the provided `IndexMap`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use indexmap::IndexMap;
    /// use credible_coin::cli::publisher::entry_map::EntryMap;
    ///
    /// let mut map = IndexMap::new();
    /// map.insert("key".to_string(), 42);
    /// let entry_map = EntryMap::from_map(map);
    /// ```
    pub fn from_map(map: IndexMap<String, i64>) -> Self {
        Self { inner: map }
    }
    /// Constructs an `EntryMap` from two separate vectors: one for keys and one for values.
    ///
    /// The vectors should be of the same length, and each key in the `key_vector` will be paired with the
    /// corresponding value in the `value_vector` based on their positions.
    ///
    /// # Arguments
    ///
    /// * `key_vector` - A vector of `String` keys.
    /// * `value_vector` - A vector of `i64` values.
    ///
    /// # Panics
    ///
    /// This method will panic if `key_vector` and `value_vector` have different lengths.
    ///
    /// # Note
    ///
    /// If a key occurs multiple times in the `key_vector`, only the last corresponding value from
    /// `value_vector` will be stored in the resulting map. As a consequence, the returned map might have
    /// fewer entries than the original vectors.
    ///
    /// # Returns
    ///
    /// Returns an `EntryMap` constructed from the provided vectors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use credible_coin::cli::publisher::entry_map::EntryMap;
    /// # use indexmap::IndexMap;
    /// let keys = vec!["key1".to_string(), "key2".to_string()];
    /// let values = vec![1, 2];
    /// let entry_map = EntryMap::from_vectors(keys, values);
    /// assert_eq!(entry_map.inner.get("key1"), Some(&1));
    /// assert_eq!(entry_map.inner.get("key2"), Some(&2));
    /// ```
    pub fn from_vectors(key_vector: Vec<String>, value_vector: Vec<i64>) -> Self {
        assert_eq!(key_vector.len(), value_vector.len());
        let mut map = IndexMap::new();
        for (pos, element) in key_vector.iter().enumerate() {
            // NOTE: If the address occurs multiple times, only the value will be updated
            // in the map so the map will have fewer keys compared to rows in the csv file
            // causing issues
            map.insert(element.clone(), value_vector[pos]);
        }
        Self { inner: map }
    }
    /// Constructs an `EntryMap` by reading addresses and values from a CSV file.
    ///
    /// The CSV file should have two columns: one for addresses (keys) and one for values. Each address in the CSV file will be paired with the corresponding value based on their positions to construct the `EntryMap`.
    ///
    /// # Arguments
    ///
    /// * `filename` - The path to the CSV file containing addresses and values.
    ///
    /// # Panics
    ///
    /// This method will panic if the addresses and values vectors obtained from the CSV file have different lengths.
    ///
    /// # Note
    ///
    /// If an address occurs multiple times in the CSV file, only the last corresponding value will be stored in the resulting map. As a consequence, the returned map might have fewer entries than the rows in the CSV file.
    ///
    /// # Returns
    ///
    /// Returns an `EntryMap` constructed from the addresses and values in the provided CSV file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use credible_coin::cli::publisher::entry_map::EntryMap;
    /// let entry_map = EntryMap::generate_address_value_map("path/to/your/test.csv");
    /// // Assert some conditions here based on your test.csv contents
    /// // Example:
    /// // assert_eq!(entry_map.inner.get("some_address_from_test_csv"), Some(&some_value_from_test_csv));
    /// ```
    pub fn generate_address_value_map(filename: &str) -> Self {
        let (addresses, values) =
            crate::utils::csv_utils::addresses_and_values_as_vectors(filename);
        println!("Address Length: {:?}", addresses.len());
        println!("Values Length: {:?}", values.len());
        EntryMap::from_vectors(addresses, values)
    }
    /// Replaces the value associated with the given address key in the `EntryMap`.
    ///
    /// If the address key is not present in the `EntryMap`, this method will panic.
    ///
    /// # Arguments
    ///
    /// * `address_key` - The address key whose value is to be replaced.
    /// * `new_val` - The new value to associate with the given address key.
    ///
    /// # Panics
    ///
    /// This method will panic if the `address_key` is not present in the `EntryMap`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use credible_coin::cli::publisher::entry_map::EntryMap;
    /// # use indexmap::IndexMap;
    /// let mut entry_map = EntryMap::from_map(IndexMap::from([(String::from("address1"), 100i64)]));
    /// entry_map.replace(String::from("address1"), 200);
    /// assert_eq!(entry_map.inner.get(&String::from("address1")), Some(&200));
    /// ```
    pub fn replace(&mut self, address_key: String, new_val: i64) {
        assert!(self.inner.contains_key(&address_key));
        self.inner
            .entry(address_key)
            .and_modify(|old_value| *old_value = new_val);
    }
}
mod tests {
    #[test]
    #[ignore = "Works fine, if we need to run this test, make sure to change the value in the file"]
    fn byte_hash_changes_after_value_update() {
        let mut cm = crate::cli::publisher::entry_map::EntryMap::generate_address_value_map(
            "BigQuery Bitcoin Historical Data - outputs.csv",
        );
        let old_value = cm
            .inner
            .get("bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k")
            .unwrap();
        assert_eq!(old_value.to_owned(), 22222);
        let old_entry = crate::merkle_tree_entry::MerkleTreeEntry::new(
            "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k".to_string(),
            *old_value,
        );
        let old_bytes = old_entry.serialize_entry();
        let old_hash = crate::merkle_tree_entry::MerkleTreeEntry::hash_bytes(old_bytes);

        cm.replace(
            "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k".to_string(),
            12345,
        );
        let retrieved_value = cm
            .inner
            .get("bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k")
            .unwrap();
        assert_eq!(retrieved_value.to_owned(), 12345);
        let entry = crate::merkle_tree_entry::MerkleTreeEntry::new(
            "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k".to_string(),
            *retrieved_value,
        );
        assert_ne!(entry.serialize_entry(), old_entry.serialize_entry());
        let bytes = entry.serialize_entry();
        let new_hash = crate::merkle_tree_entry::MerkleTreeEntry::hash_bytes(bytes);
        assert_ne!(old_hash, new_hash);
    }
    #[test]
    fn bytes_equality() {
        let mut address: String = "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k".to_owned();
        // &address.push_str(&22222.to_string());
        // assert_eq!(address, "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k22222".to_owned());
        address.push_str(&22222.to_string());
        let first = <rs_merkle::algorithms::Sha256 as rs_merkle::Hasher>::hash(
            &bincode::serialize(&address).unwrap(),
        );

        let mut new_address: String = "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k".to_owned();
        new_address.push_str(&12345.to_string());
        let second = <rs_merkle::algorithms::Sha256 as rs_merkle::Hasher>::hash(
            &bincode::serialize(&new_address).unwrap(),
        );

        assert_ne!(first, second);
    }
}
