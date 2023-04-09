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
    pub fn new(coin_address: String, value: i64) -> Self {
        return Self {
            coin_address,
            value,
        };
    }
    pub fn coin_value(&self) -> i64 {
        return self.value;
    }
    pub fn coin_address(&self) -> String {
        return self.coin_address.clone();
    }
    pub fn create_coin_vector(addresses: Vec<String>, values: Vec<i64>) -> Vec<Coin> {
        assert_eq!(addresses.len(), values.len());
        let mut coins: Vec<Coin> = Vec::new();
        for (a, v) in addresses.iter().zip(values.iter()) {
            println!("Address={}, Value={}", a, v);
            coins.push(Coin::new(a.to_string(), *v))
        }
        return coins;
    }
}
