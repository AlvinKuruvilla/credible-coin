//! A simple wrapper type representing a piece of cryptocurrency
//!
//! Contains a String value and a String address mostly mirroring how transactions are
//! reflected in a cryptocurrency-system with a blockchain ledger
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
            println!("Address={}, Value={}", a, v);
            coins.push(Coin::new(a.to_string(), *v))
        }
        return coins;
    }
    /// Serialize a coin into bytes
    ///
    /// The serialization algorithm just takes the address String and concatenates it to
    /// the coin value string and uses bincode::serialize on it
    pub fn serialize_coin(&self) -> Vec<u8> {
        self.coin_address().push_str(&self.coin_value().to_string());
        return bincode::serialize(&self.coin_address()).unwrap();
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
}
