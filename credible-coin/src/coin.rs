//! A simple wrapper type representing a u32 value
#[derive(Debug, Copy, Clone, Default)]
pub struct Coin {
    value: u32,
}
impl Coin {
    pub fn new(&self) -> Self {
        return Self::default();
    }
}
