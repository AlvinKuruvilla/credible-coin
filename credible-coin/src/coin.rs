//! A simple wrapper type representing a piece of cryptocurrency
//!
//! Contains a String value and a String address mostly mirroring how transactions are
//! reflected in a cryptocurrency-system with a blockchain ledger
use rs_merkle::{algorithms::Sha256, Hasher};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Coin {
    coin_address: String,
    value: i64,
}
impl Coin {
    /// Construct a new coin from a given address string and value
    pub fn new(coin_address: String, value: i64) -> Self {
        return Self {
            coin_address,
            value,
        };
    }
    /// Get the value for the coin
    pub fn coin_value(&self) -> i64 {
        return self.value;
    }
    /// Get the address for the coin
    pub fn coin_address(&self) -> String {
        return self.coin_address.clone();
    }
    /// Given a vector of Strings and i64 values construct a Ve<Coin> using each (String, i64) pair
    pub fn create_coin_vector(addresses: Vec<String>, values: Vec<i64>) -> Vec<Coin> {
        assert_eq!(addresses.len(), values.len());
        let mut coins: Vec<Coin> = Vec::new();
        for (a, v) in addresses.iter().zip(values.iter()) {
            println!("Address= {}, Value= {}", a, v);
            coins.push(Coin::new(a.to_string(), *v))
        }
        println!();
        println!("{:?}", coins.len());
        return coins;
    }
    /// Serialize a coin into bytes
    ///
    /// The serialization algorithm just takes the address String and concatenates it to
    /// the coin value string and uses bincode::serialize on it
    pub fn serialize_coin(&self) -> Vec<u8> {
        let res = format!("{}{}", self.coin_address(), &self.coin_value().to_string());
        return bincode::serialize(&res).unwrap();
    }
    /// Take the given vector of u8's iterate each element and turn into bytes, hash it,
    /// and then collect into a new vector
    pub fn hash_bytes(bytevector: Vec<u8>) -> [u8; 32] {
        let leaves: [u8; 32] = Sha256::hash(&bytevector);
        return leaves;
    }

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let address = "17wNSD33wQFDwMnzUHRPCsXseWctUZVQEC";
        let value = 72160;
        let coin = Coin::new(address.to_string(), value);
        let coin_bytes = bincode::serialize(&coin).unwrap();
        coin.coin_address().push_str(&coin.coin_value().to_string());
        let distinct_bytes = bincode::serialize(&coin.coin_address()).unwrap();
        assert_ne!(coin_bytes, distinct_bytes);
    }
    #[test]
    pub fn combine_address_and_value() {
        let mut address: String = "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k".to_owned();
        address.push_str(&22222.to_string());
        assert_eq!(address, "bc1qushqa4nwpz2j0yftnpw08c5lj2u92mnah79q2k22222".to_owned());
    }
}
