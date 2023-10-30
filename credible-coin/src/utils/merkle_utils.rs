use rs_merkle::{algorithms::Sha256, MerkleTree};

use crate::{
    cli::publisher::entry_map::EntryMap, merkle_tree_entry::MerkleTreeEntry,
    utils::csv_utils::addresses_and_values_as_vectors, utils::csv_utils::get_address_position,
};
use anyhow::{anyhow, Result};

/// Creates leaves from coin vectors
pub fn load_merkle_leaves_from_csv(file_name: &str) -> Vec<[u8; 32]> {
    let (v1, v2) = addresses_and_values_as_vectors(file_name);
    let vec_entries = MerkleTreeEntry::create_entries_vector(v1, v2);

    let mut serialized_entries: Vec<Vec<u8>> = Vec::new();

    for entry in vec_entries {
        serialized_entries.push(entry.serialize_entry());
    }
    let mut leaves_vec: Vec<[u8; 32]> = Vec::new();
    for entry in serialized_entries {
        leaves_vec.push(MerkleTreeEntry::hash_bytes(entry));
    }
    leaves_vec
}

/// Prove that a coin is a member of the merkle tree given its public address and an optional value
/// Note that the value is only needed by the publisher shell because ledger
/// entries can use the same address multiple times, so we use the value to distinguish them
// TODO: What is the best way to handle duplicate delta values in the ledger file?
pub fn prove_membership(
    filename: &str,
    _public_address: &str,
    value: Option<i64>,
    tree: &MerkleTree<Sha256>,
) -> Result<()> {
    let tree_leaves = tree
        .leaves()
        .ok_or_else(|| anyhow!("Could not get leaves to prove"))?;
    let map = EntryMap::generate_address_value_map(filename);

    let map_value = map
        .inner
        .get(_public_address)
        .ok_or_else(|| anyhow!("Could not find public address {:?}", _public_address))?;

    let (generated_entry, address_index) = if let Some(value) = value {
        let index = get_address_position(filename, _public_address.to_string(), Some(value))
            .map_err(|e| {
                anyhow!(
                    "Could not get address position with provided value: {:?}",
                    e
                )
            })?;
        (
            MerkleTreeEntry::new(_public_address.to_owned(), value),
            index,
        )
    } else {
        let index: usize =
            get_address_position(filename, _public_address.to_string(), Some(*map_value))
                .map_err(|e| anyhow!("Could not get address position with map value: {:?}", e))?;
        (
            MerkleTreeEntry::new(_public_address.to_owned(), *map_value),
            index,
        )
    };

    let indices = vec![address_index];
    let proof = tree.proof(&indices);
    let root = tree
        .root()
        .ok_or_else(|| anyhow!("Couldn't get the merkle root"))?;
    let bytes = generated_entry.serialize_entry();
    let hashed_bytes = [MerkleTreeEntry::hash_bytes(bytes)];

    if !proof.verify(root, &indices, &hashed_bytes, tree_leaves.len()) {
        return Err(anyhow!("Verification failed"));
    }

    log::info!("Address {:?} found in merkle tree", _public_address);
    Ok(())
}
