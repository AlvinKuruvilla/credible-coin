//!
//! CredibleCoin is a zero-knowledge proof system to asses cryptocurrency exchange solvency.
//! Currently the plan is to implement this system for Bitcoin, however, this crate should be built so
//! that it should relatively easy to implement for other cryptocurrency exchanges

pub mod accumulator;
pub mod coin;
pub mod exchange;
