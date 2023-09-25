//! A simple wrapper type representing a piece of cryptocurrency
//!
//! Contains a `i64` value and a `String` address
//! This could represent a transaction on the public blockchain ledger (used by the publisher client)
//! or it could also represent a set of accounts and their associated account values (used by the exchange client to maintain a secret set).
//! In either case, these entries end up in a merkle tree.

use rs_merkle::{algorithms::Sha256, Hasher};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MerkleTreeEntry {
    coin_address: String,
    value: i64,
}
impl MerkleTreeEntry {
    /// Construct a new ```MerkleTreeEntry``` from a given address string and value
    pub fn new(coin_address: String, value: i64) -> Self {
        Self {
            coin_address,
            value,
        }
    }
    /// Get the value for the MerkleTreeEntry
    pub fn entry_value(&self) -> i64 {
        self.value
    }
    /// Get the address for the MerkleTreeEntry
    pub fn entry_address(&self) -> String {
        self.coin_address.clone()
    }
    /// Given a vector of Strings and i64 values construct a `Vec<MerkleTreeEntry>` using each (String, i64) pair
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

    /// Serialize a MerkleTreeEntry into bytes
    ///
    /// The serialization algorithm just takes the address String and concatenates it to
    /// the MerkleTreeEntry value string and uses `bincode::serialize` on it
    pub fn serialize_entry(&self) -> Vec<u8> {
        let res = format!(
            "{}{}",
            self.entry_address(),
            &self.entry_value().to_string()
        );
        bincode::serialize(&res).unwrap()
    }
    /// Take the given vector of u8's iterate each element and turn into bytes, hash it,
    /// and then collect into a new vector
    pub fn hash_bytes(bytevector: Vec<u8>) -> [u8; 32] {
        Sha256::hash(&bytevector)
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
    pub fn combine_address_and_value() {
        let mut address: String = "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k".to_owned();
        address.push_str(&22222.to_string());
        assert_eq!(
            address,
            "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k22222".to_owned()
        );
    }
}
