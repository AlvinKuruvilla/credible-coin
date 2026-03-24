use super::{AbstractAccumulator, MembershipProof};
use crate::{
    credible_config::{get_circuit_file_path, get_entries_file_path},
    merkle_tree_entry::MerkleTreeEntry,
    utils::csv_utils::get_address_position,
};
use anyhow::Result;
use emp_rs::merkle::{prove_membership_from_file, MerkleTree};
use rayon::{
    iter::ParallelBridge,
    prelude::{IntoParallelRefIterator, ParallelIterator},
};
use std::collections::HashMap;
use std::path::Path;

/// In cryptographic protocols, an accumulator is a primitive that allows you to
/// represent a set of elements and prove membership (or non-membership) without
/// revealing which elements are in the set A Delta Accumulator is a variation
/// on that idea but having a secret set of address and value pairs representing
/// the exchange's secret set Then using the public ledger, the accumulator runs
/// a membership on each of its entries finding all of the entries the exchange
/// can prove membership of and gets their values This value becomes the value
/// of the exchange's liabilities
#[derive(Debug, Clone)]
pub struct DeltaAccumulator {
    /// The path to the exchange secrets file
    pub exchange_secrets_path: String,
}
impl AbstractAccumulator for DeltaAccumulator {
    fn prove_member(
        &self,
        _element: &MerkleTreeEntry,
        pos: Option<usize>,
    ) -> Result<MembershipProof> {
        let pos = pos.ok_or_else(|| anyhow::anyhow!("Leaf position is required"))?;

        let entries_file = get_entries_file_path();
        let circuit_file = get_circuit_file_path();

        let proof = prove_membership_from_file(&entries_file, &circuit_file, pos)
            .map_err(|e| anyhow::anyhow!("emp_rs proof error: {}", e))?;

        Ok(MembershipProof {
            is_member: proof.is_member,
        })
    }

    fn verify(&self, _element_proof: MembershipProof) {
        panic!("This function should not be called");
    }
    fn search(&self, entry: &MerkleTreeEntry) -> Result<usize> {
        println!(
            "{:?}, {:?}: {:?}",
            entry.entry_address(),
            entry.entry_value(),
            get_address_position(&self.exchange_secrets_path, entry.entry_address(), None)?
        );
        Ok(get_address_position(
            &self.exchange_secrets_path,
            entry.entry_address(),
            None,
        )?)
    }
    fn aggregate(&self, ledger_file: String, ledger_entries: Vec<MerkleTreeEntry>) -> Result<i64> {
        let matching_entries_map = self.precompute_matching_entries(&ledger_entries);
        let delta = std::sync::Arc::new(std::sync::atomic::AtomicI64::new(0));

        let entries_file = get_entries_file_path();
        let circuit_file = get_circuit_file_path();
        let tree = MerkleTree::from_file(Path::new(&entries_file), Path::new(&circuit_file))
            .map_err(|e| anyhow::anyhow!("Failed to build Merkle tree: {}", e))?;

        matching_entries_map
            .values()
            .flatten()
            .enumerate()
            .par_bridge()
            .for_each(|(entry_index, entry_match)| {
                let delta_clone = std::sync::Arc::clone(&delta);

                println!("Index {}: Processing entry.", entry_index);

                let pos = match get_address_position(
                    &ledger_file,
                    entry_match.entry_address(),
                    Some(entry_match.entry_value()),
                ) {
                    Ok(p) => p,
                    Err(e) => {
                        println!("Failed to get address position: {:?}", e);
                        return;
                    }
                };

                let leaf_index = pos + 1;
                if let Some(leaf) = tree.get_leaf(leaf_index) {
                    let (result, _path) = tree.prove_in_tree(leaf);
                    if result == 2 {
                        let value_to_add = entry_match.entry_value();
                        println!("Adding value: {:?}", value_to_add);
                        delta_clone.fetch_add(value_to_add, std::sync::atomic::Ordering::SeqCst);
                    } else {
                        println!("Entry is not a member: {:?}", entry_match);
                    }
                } else {
                    println!("Leaf index {} out of bounds", leaf_index);
                }
            });

        Ok(delta.load(std::sync::atomic::Ordering::SeqCst))
    }
}
impl DeltaAccumulator {
    /// Make a new `DeltaAccumulator` from the provided exchange_path string
    pub fn new(exchange_path: String) -> Self {
        Self {
            exchange_secrets_path: exchange_path,
        }
    }
    /// Returns all `MerkleTreeEntry` items from the provided ledger entries that match the specified address.
    pub fn get_all_matching_address_entries(
        &self,
        ledger_entries: &[MerkleTreeEntry],
        address: &str,
    ) -> Vec<MerkleTreeEntry> {
        ledger_entries
            .par_iter()
            .filter(|entry| entry.entry_address() == address)
            .cloned()
            .collect()
    }
    /// Precomputes and groups ledger entries by their address.
    pub fn precompute_matching_entries(
        &self,
        ledger_entries: &[MerkleTreeEntry],
    ) -> HashMap<String, Vec<MerkleTreeEntry>> {
        let mut map: HashMap<String, Vec<MerkleTreeEntry>> = HashMap::new();

        for entry in ledger_entries {
            map.entry(entry.entry_address())
                .or_default()
                .push(entry.clone());
        }

        map
    }
}
