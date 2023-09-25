use rs_merkle::{algorithms::Sha256, MerkleTree};

use crate::{
    cli::publisher::entry_map::EntryMap, merkle_tree_entry::MerkleTreeEntry,
    utils::csv_utils::addresses_and_values_as_vectors, utils::csv_utils::get_address_position,
};

/// Creates leaves from coin vectors
pub fn load_merkle_leaves_from_csv(file_name: &str) -> Vec<[u8; 32]> {
    let (v1, v2) = addresses_and_values_as_vectors(file_name);
    let vec_entries = MerkleTreeEntry::create_entries_vector(v1, v2);

    let mut serialized_entries: Vec<Vec<u8>> = Vec::new();

    for entry in vec_entries {
        serialized_entries.push(entry.serialize_entry());
    }
    // println!("{:?}", u8coins);
    // std::thread::sleep(std::time::Duration::from_millis(100000));
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
) {
    // FIXME: We should make this logic generic so we don't need to
    // rewrite the code for a specific dataset
    let tree_leaves = tree
        .leaves()
        .ok_or("Could not get leaves to prove")
        .unwrap();
    let map = EntryMap::generate_address_value_map(filename);
    let mut address_index = 0;
    let mut generated_entry = MerkleTreeEntry::default();
    let map_value = if let Some(v) = map.inner.get(_public_address) {
        // dbg!("Using public address {:?}", _public_address);
        v
    } else {
        log::error!("Could not find public address {:?}", _public_address);
        return;
    };
    // dbg!("Using value {:?}", *map_value);
    if value.is_none() {
        generated_entry = MerkleTreeEntry::new(_public_address.to_owned(), *map_value);
        address_index =
            get_address_position(filename, _public_address.to_string(), Some(*map_value));
    } else {
        address_index = get_address_position(filename, _public_address.to_string(), value);
        generated_entry = MerkleTreeEntry::new(_public_address.to_owned(), value.unwrap());
    }
    // dbg!("Address Position {:?}", address_index);
    let indices = vec![address_index];
    let proof = tree.proof(&indices);
    let root = tree.root().ok_or("couldn't get the merkle root").unwrap();
    let bytes = generated_entry.serialize_entry();
    let hashed_bytes = [MerkleTreeEntry::hash_bytes(bytes)];

    assert!(proof.verify(root, &indices, &hashed_bytes, tree_leaves.len()));
    log::info!("Address {:?} found in merkle tree", _public_address);
}
