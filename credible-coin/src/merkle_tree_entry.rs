//! A simple wrapper type representing a piece of cryptocurrency
//!
//! Contains a [`i64`] value and a [`String`] address This could represent a
//! transaction on the public blockchain ledger (used by the publisher client)
//! or it could also represent a set of accounts and their associated account
//! values (used by the exchange client to maintain a secret set). In either
//! case, these entries end up in a merkle tree.

use std::fmt::{self, Display};

use rs_merkle::{algorithms::Sha256, Hasher};
use serde::{Deserialize, Serialize};
/// A simple wrapper type representing a piece of cryptocurrency
///
/// Contains a [`i64`] value and a [`String`] address This could represent a
/// transaction on the public blockchain ledger (used by the publisher client)
/// or it could also represent a set of accounts and their associated account
/// values (used by the exchange client to maintain a secret set). In either
/// case, these entries end up in a merkle tree.

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, Hash)]
pub struct MerkleTreeEntry {
    coin_address: String,
    value: i64,
}
impl MerkleTreeEntry {
    /// Construct a new [`MerkleTreeEntry`] from a given address string and value.
    ///
    /// # Arguments
    ///
    /// * `coin_address` - A string representing the coin's address.
    /// * `value` - The value associated with the coin address.
    ///
    /// # Returns
    ///
    /// A new `MerkleTreeEntry` containing the provided coin address and value.
    ///
    /// # Examples
    ///
    /// ```
    /// use credible_coin::merkle_tree_entry::MerkleTreeEntry;
    ///
    /// let coin_address = String::from("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
    /// let value = 1000;
    ///
    /// let entry = MerkleTreeEntry::new(coin_address, value);
    /// ```

    pub fn new(coin_address: String, value: i64) -> Self {
        Self {
            coin_address,
            value,
        }
    }
    /// Get the value associated with the [`MerkleTreeEntry`].
    ///
    /// This method returns the value of the `MerkleTreeEntry`.
    ///
    /// # Returns
    ///
    /// The value associated with the `MerkleTreeEntry`.
    ///
    /// # Examples
    ///
    /// ```
    /// use credible_coin::merkle_tree_entry::MerkleTreeEntry;
    ///
    /// let coin_address = String::from("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
    /// let value = 1000;
    ///
    /// let entry = MerkleTreeEntry::new(coin_address.clone(), value);
    /// let retrieved_value = entry.entry_value();
    ///
    /// assert_eq!(retrieved_value, value);
    /// ```
    #[inline]
    pub fn entry_value(&self) -> i64 {
        self.value
    }

    /// Get the address associated with the [`MerkleTreeEntry`].
    ///
    /// This method returns a clone of the coin address of the `MerkleTreeEntry`.
    ///
    /// # Returns
    ///
    /// A cloned `String` containing the coin address.
    ///
    /// # Examples
    ///
    /// ```
    /// use credible_coin::merkle_tree_entry::MerkleTreeEntry;
    ///
    /// let coin_address = String::from("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
    /// let value = 1000;
    ///
    /// let entry = MerkleTreeEntry::new(coin_address.clone(), value);
    /// let retrieved_address = entry.entry_address();
    ///
    /// assert_eq!(retrieved_address, coin_address);
    /// ```
    #[inline]
    pub fn entry_address(&self) -> String {
        self.coin_address.clone()
    }
    /// Given a [`Vec<String>`] and [`Vec<i64>`] values construct a
    /// [`Vec<MerkleTreeEntry>`] using each [`(String, i64)`] pair
    pub fn create_entries_vector(addresses: Vec<String>, values: Vec<i64>) -> Vec<MerkleTreeEntry> {
        assert_eq!(addresses.len(), values.len());
        let mut entries: Vec<MerkleTreeEntry> = Vec::new();
        for (a, v) in addresses.iter().zip(values.iter()) {
            // println!("Address= {}, Value= {}", a, v);
            entries.push(MerkleTreeEntry::new(a.to_string(), *v));
        }
        println!("Address Count: {:?}", entries.len());
        entries
    }

    /// Serialize a `MerkleTreeEntry` into bytes.
    ///
    /// The serialization algorithm concatenates the address string and the value as a string
    /// and then uses [`bincode::serialize`] to serialize it into a byte vector.
    ///
    /// # Returns
    ///
    /// A byte vector representing the serialized `MerkleTreeEntry`.
    ///
    /// # Examples
    ///
    /// ```
    /// use credible_coin::merkle_tree_entry::MerkleTreeEntry;
    ///
    /// let coin_address = String::from("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
    /// let value = 1000;
    ///
    /// let entry = MerkleTreeEntry::new(coin_address.clone(), value);
    /// let serialized_entry = entry.serialize_entry();
    /// ```
    pub fn serialize_entry(&self) -> Vec<u8> {
        let res = format!(
            "{}{}",
            self.entry_address(),
            &self.entry_value().to_string()
        );
        bincode::serialize(&res).unwrap()
    }

    /// Hash a vector of [`u8`] elements using SHA-256.
    ///
    /// This method takes a vector of bytes, hashes them using the SHA-256 algorithm,
    /// and returns the resulting hash as a fixed-size array of 32 bytes.
    ///
    /// # Arguments
    ///
    /// * `bytevector` - A vector of bytes to be hashed.
    ///
    /// # Returns
    ///
    /// A 32-byte array representing the SHA-256 hash of the input bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use credible_coin::merkle_tree_entry::MerkleTreeEntry;
    ///
    /// let bytevector = vec![0, 1, 2, 3, 4, 5];
    /// let hash = MerkleTreeEntry::hash_bytes(bytevector);
    /// ```
    pub fn hash_bytes(bytevector: Vec<u8>) -> [u8; 32] {
        Sha256::hash(&bytevector)
    }
}
impl Display for MerkleTreeEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address: {}, Value: {}", self.coin_address, self.value)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let address = "17wNSD33wQFDwMnzUHRPCsXseWctUZVQEC";
        let value = 72160;
        let entry = MerkleTreeEntry::new(address.to_string(), value);
        let coin_bytes = bincode::serialize(&entry).unwrap();
        entry
            .entry_address()
            .push_str(&entry.entry_value().to_string());
        let distinct_bytes = bincode::serialize(&entry.entry_address()).unwrap();
        assert_ne!(coin_bytes, distinct_bytes);
    }
    #[test]
    fn combine_address_and_value() {
        let mut address: String = "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k".to_owned();
        address.push_str(&22222.to_string());
        assert_eq!(
            address,
            "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k22222".to_owned()
        );
    }
}
