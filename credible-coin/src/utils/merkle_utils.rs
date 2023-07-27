use rs_merkle::{algorithms::Sha256, MerkleTree};

use crate::{
    cli::publisher::coin_map::CoinMap, coin::Coin, utils::csv_utils::get_address_position,
};

use super::csv_utils::addresses_and_values_as_vectors;

/// Creates leaves from coin vectors
pub fn load_merkle_leaves(file_name: &str) -> Vec<[u8; 32]> {
    let (v1, v2) = addresses_and_values_as_vectors(file_name);
    let vec_coin = Coin::create_coin_vector(v1, v2);

    // for c in vec_coin.iter() {
    //     println!("Bytes= {:?}", c.serialize_coin());
    // }

    let mut u8coins: Vec<Vec<u8>> = Vec::new();

    for coin in vec_coin {
        u8coins.push(coin.serialize_coin());
    }
    // println!("{:?}", u8coins);
    // std::thread::sleep(std::time::Duration::from_millis(100000));
    let mut leaves_vec: Vec<[u8; 32]> = Vec::new();
    for coin in u8coins {
        leaves_vec.push(Coin::hash_bytes(coin))
    }
    return leaves_vec;
}

/// Prove that a coin is a member of the merkle tree given its public address
pub fn prove_membership(filename: &str, _public_address: &str, tree: &MerkleTree<Sha256>) {
    let tree_leaves = tree
        .leaves()
        .ok_or("Could not get leaves to prove")
        .unwrap();
    let map = CoinMap::generate_address_value_map(filename);
    let value = match map.inner.get(_public_address) {
        Some(v) => v,
        None => {
            log::error!("Could not find public address {:?}", _public_address);
            return;
        }
    };
    let generated_coin = Coin::new(_public_address.to_owned(), *value);
    let address_index = get_address_position(filename, _public_address.to_string());
    let indices = vec![address_index];
    let proof = tree.proof(&indices);
    let root = tree.root().ok_or("couldn't get the merkle root").unwrap();
    let bytes = generated_coin.serialize_coin();
    let hashed_bytes = [Coin::hash_bytes(bytes)];

    assert!(proof.verify(root, &indices, &hashed_bytes, tree_leaves.len()));
    log::info!("Address {:?} found in merkle tree", _public_address);
}
