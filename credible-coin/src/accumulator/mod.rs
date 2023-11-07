//! An accumulator is a one way hash function that certifies that candidates are a member of a set without revealing the individual members of the set.
//!
//! This crate currently implements:
//! 1. [`Delta Accumulator`](crate::accumulator::value_delta::DeltaAccumulator)

use crate::merkle_tree_entry::MerkleTreeEntry;
use anyhow::Result;
use serde::{Deserialize, Serialize};
/// A blanket struct type representing a "proof of membership".
/// A membership proof is an interactive proof for a statement of the form x in L, where L is some formal language.
/// The only information held in the type is whether the statement is a member of the language (ie the element is
/// part of the secret set, if considering from the perspective of a membership proof)
#[derive(Serialize, Deserialize, Debug)]
pub struct MembershipProof {
    is_member: bool,
}
/// Common Functionality an accumulator should have
pub trait AbstractAccumulator {
    /// Prove that a [`Merkle Tree
    /// Entry`](`crate::merkle_tree_entry::MerkleTreeEntry`) is a member of a
    /// particular set by generating a [`Membership Proof`](MembershipProof),
    /// optionally providing a position for the entry. These merkle tree entries
    /// could from a custom proof backend like emp-zk or an existing crate like
    /// [`rs_merkle`].
    fn prove_member(
        &self,
        element: &MerkleTreeEntry,
        pos: Option<usize>,
    ) -> Result<MembershipProof>;
    /// Verify the provided [`Membership Proof`](MembershipProof)
    fn verify(&self, element_proof: MembershipProof);
    /// Search for a particular [`Merkle Tree Entry`](MerkleTreeEntry) and
    /// return it's position in the file
    fn search(&self, entry: &MerkleTreeEntry) -> anyhow::Result<usize>;
    /// Aggregate the final delta using the public ledger's entries and the
    /// entries from the exchange's secret set This function follows a 2-step
    /// process to perform the delta aggregation:
    /// 1. Search for each ledger address against the exchange's secret set,
    /// keeping only the addresses in both
    /// 2. For each relevant address then prove membership in the exchange set,
    /// using the above prove_member function The delta is only accumulated for
    /// those addresses in which the membership proof is true (ie they are part
    /// of the set)
    fn aggregate(&self, ledger: String, ledger_entries: Vec<MerkleTreeEntry>) -> Result<i64>;
}
/// Our custom implementation of a delta accumulation proof using emp-zk as a
/// zero-knowledge backend
pub mod value_delta;
