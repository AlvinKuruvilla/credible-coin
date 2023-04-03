//! A simple wrapper type representing a piece of cryptocurrency
//!
//! Contains a String value and a String address mostly mirroring how transactions are
//! reflected in a cryptocurrency-system with a blockchain ledger

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Coin {
    coin_address: String,
    value: String,
}
impl Coin {
    pub fn new(&self, coin_address: String, value: String) -> Self {
        return Self {
            coin_address,
            value,
        };
    }
    pub fn coin_value(&self) -> String {
        return self.value.clone();
    }
    pub fn coin_address(&self) -> String {
        return self.coin_address.clone();
    }
}
