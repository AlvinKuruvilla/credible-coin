//! An accumulator is a one way hash function that certifies that candidates are a member of a set without revealing the individual members of the set.
//!
//! This crate implements 2 types of accumulators:
//! 1. Merkle Trees
//! 2. Binary Accumulators

use crate::merkle_tree_entry::MerkleTreeEntry;
use anyhow::Result;
/// A blanket struct type representing a "proof of membership".
/// A membership proof is an interactive proof for a statement of the form x in L, where L is some formal language.
/// The only information held in the type is whether the statement is a member of the language (ie the element is
/// part of the secret set, if considering from the perspective of a membership proof)
pub struct MembershipProof {
    is_member: bool,
}
/// Common Functionality an accumulator should have
pub trait AbstractAccumulator {
    /// Prove that a [`coin::Coin`] is a member of a particular [`MembershipProof`]. For example,
    /// we have 2 concrete implementors of the [`MembershipProof`] trait
    /// 1. [`rs_merkle::MerkleProof`]
    /// 2. `BinaryAccumulatorProof`
    /// By providing a Generic parameter `M` on the function signature we specify that we will return a type `M` which has this trait as its bound (so either  [`rs_merkle::MerkleProof`] or `BinaryAccumulatorProof`)
    fn prove_member(&self, element: MerkleTreeEntry) -> Result<MembershipProof>;
    /// Verify the proof of any type implementing [`MembershipProof`]
    fn verify(&self, element_proof: MembershipProof);
    /// Search for a particular [`MerkleTreeEntry`] and return it's position in the file
    fn search(&self, entry: MerkleTreeEntry) -> anyhow::Result<usize>;
    /// Aggregate the final delta using the public ledger's entries and the entries from the exchange's secret set
    /// This function follows a 2-step process to perform the delta aggregation:
    /// 1. Search for each ledger address against the exchange's secret set, keeping only the addresses
    /// in both
    /// 2. For each relevant address then prove membership in the exchange set, using the above prove_member function
    /// The delta is only accumulated for those addresses in which the membership proof is true (ie they are part of the set)
    fn aggregate(
        &self,
        ledger: Vec<MerkleTreeEntry>,
        exchange_entries: Vec<MerkleTreeEntry>,
    ) -> Result<i64>;
}
pub mod value_delta;
