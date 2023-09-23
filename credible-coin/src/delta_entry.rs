//! A type representing a bitcoin address and its associated value delta
//! as would be seen in the public blockchain ledger
use rs_merkle::{algorithms::Sha256, Hasher};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Delta {
    address: String,
    delta_value: i64,
}
impl Delta {
    /// Construct a new coin from a given address string and value
    pub fn new(address: String, value: i64) -> Self {
        Self {
            address,
            delta_value: value,
        }
    }
    /// Get the delta value for the delta entry
    pub fn delta_value(&self) -> i64 {
        self.delta_value
    }
    /// Get the address for the delta entry
    pub fn address(&self) -> String {
        self.address.clone()
    }
    /// Serialize a delta entry into bytes
    ///
    /// The serialization algorithm just takes the address String and concatenates it to
    /// the delta string and uses `bincode::serialize` on it
    pub fn serialize_entry(&self) -> Vec<u8> {
        let res = format!("{}{}", self.address(), &self.delta_value().to_string());
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
        let delta = Delta::new(address.to_string(), value);
        let bytes = bincode::serialize(&delta).unwrap();
        delta.address().push_str(&delta.delta_value().to_string());
        let distinct_bytes = bincode::serialize(&delta.address()).unwrap();
        assert_ne!(bytes, distinct_bytes);
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
