//! A simple wrapper type representing a u32 value

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
