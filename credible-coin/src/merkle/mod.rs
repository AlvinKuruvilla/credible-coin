use crate::coin::Coin;
use serde::{Deserialize, Serialize};
use rs_merkle::{algorithms::Sha256, Hasher};
// TODO: We need a bunch of tests for this but I'm not sure how we go about it

/// A MrkleNode is a wrapper for a Coin. 
/// A Coin contains the address and the value 
#[derive(Serialize, Deserialize)]
pub struct MerkleNode {
    coin: Coin,
}
impl MerkleNode {
    /// Convert the given reference to the struct into a vector u8's
    pub fn into_bytevec(&self) -> Vec<u8>{
        return bincode::serialize(&self).unwrap();
    }
}
/// Converts a vector of bytes back into the struct
pub fn from_bytes_into_struct(encoded: Vec<u8> ) {
    return bincode::deserialize(&encoded[..]).unwrap();
}
/// Take the given vector of u8's iterate each element and turn into bytes, hash it, 
/// and then collect into a new vector
pub fn hash_bytes(bytevector: Vec<u8>) -> Vec<[u8; 32]> {
    let leaves: Vec<[u8; 32]> = bytevector.iter().map(|x|Sha256::hash(&x.to_ne_bytes())).collect();
    return leaves;
}