use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};

use crate::{
    cli::publisher::coin_map::CoinMap, coin::Coin,
    utils::csv_utils::addresses_and_values_as_vectors, utils::csv_utils::get_address_position,
};

/// Creates leaves from coin vectors
pub fn load_merkle_leaves_from_csv(file_name: &str) -> Vec<[u8; 32]> {
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
        leaves_vec.push(Coin::hash_bytes(coin));
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
    let map = CoinMap::generate_address_value_map(filename);
    let mut address_index = 0;
    let mut generated_coin = Coin::default();
    let map_value = if let Some(v) = map.inner.get(_public_address) {
        // dbg!("Using public address {:?}", _public_address);
        v
    } else {
        log::error!("Could not find public address {:?}", _public_address);
        return;
    };
    // dbg!("Using value {:?}", *map_value);
    if value.is_none() {
        generated_coin = Coin::new(_public_address.to_owned(), *map_value);
        address_index =
            get_address_position(filename, _public_address.to_string(), Some(*map_value));
    } else {
        address_index = get_address_position(filename, _public_address.to_string(), value);
        generated_coin = Coin::new(_public_address.to_owned(), value.unwrap());
    }
    // dbg!("Address Position {:?}", address_index);
    let indices = vec![address_index];
    let proof = tree.proof(&indices);
    let root = tree.root().ok_or("couldn't get the merkle root").unwrap();
    let bytes = generated_coin.serialize_coin();
    let hashed_bytes = [Coin::hash_bytes(bytes)];

    assert!(proof.verify(root, &indices, &hashed_bytes, tree_leaves.len()));
    log::info!("Address {:?} found in merkle tree", _public_address);
}
