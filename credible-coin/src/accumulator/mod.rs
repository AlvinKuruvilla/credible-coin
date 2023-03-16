//! An accumulator is a one way hash function that certifies that candidates are a member of a set without revealing the individual members of the set.
//!
//! This crate implements 2 types of accumulators:
//! 1. Merkle Trees
//! 2. Binary Accumulators
use crate::coin;

/// A set of helper functions to convert to convert Vector and slice types
/// to Vec<[N;u8]> where N is the size of the slice
pub mod uint_typecast;
/// A blanket trait type representing a "proof of membership". A membership proof is an interactive proof for a statement of the form x in L, where L is some formal language.
pub trait MembershipProof {}
/// Common Functionality an accumulator should have
pub trait AbstractAccumulator {
    /// Prove that a [`coin::Coin`] is a member of a particular [`MembershipProof`]. For example,
    /// we have 2 concrete implementors of the [`MembershipProof`] trait
    /// 1. [`MerkleProof`]
    /// 2. [`BinaryAccumulatorProof`]
    /// By providing a Generic parameter `M` on the function signature we specify that we will return a type `M` which has this trait as its bound (so either  [`MerkleProof`] or [`BinaryAccumulatorProof`])
    fn prove_member<M: MembershipProof>(element: coin::Coin) -> M;
    /// Verify the proof of any type implementing [`MembershipProof`]
    fn verify<M: MembershipProof>(element_proof: M);
    /// Search for an return a particular [`coin::Coin`] and return it
    fn search(coin: coin::Coin) -> coin::Coin;
}
