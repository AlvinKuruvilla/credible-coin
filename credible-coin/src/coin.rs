//! A simple wrapper type representing a u32 value

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Coin {
    value: String,
    coin_address: String,
}
impl Coin {
    pub fn new(&self) -> Self {
        return Self::default();
    }
    pub fn coin_value(&self) -> String{
        return self.value.clone();
    }
    pub fn coin_address(&self) -> String{
        return self.coin_address.clone();
    }
}
