use crate::coin::*;
use crate::merkle::*;
use crate::utils::address_generator::*;
use crate::utils::csv_utils::*;
use polars::prelude::*;
use rs_merkle::{algorithms::Sha256, MerkleTree};
use std::path::Path;
//creates csv file from random addresses and values
pub fn create_db(FILENAME :&str, row_count: u32) {
    assert!(!Path::new(FILENAME)
            .try_exists()
            .expect("file already exists"), "file already exists");
    let mut datafr = generate_n_address_value_dataframe(row_count);
    let mut file = std::fs::File::create(FILENAME).unwrap();
    CsvWriter::new(&mut file).finish(&mut datafr).unwrap();
}

//creates leaves from coin vectors
pub fn load_merkle_leaves(file_name: &str) -> Vec<[u8; 32]> {
    let (v1, v2) = addresses_and_values_as_vectors(file_name);
    let vec_coin = Coin::create_coin_vector(v2, v1);
    let vec_nodes: Vec<MerkleNode> = from_vec_coins_to_vec_nodes(vec_coin);

    let mut u8coins: Vec<Vec<u8>> = Vec::new();

    for node in vec_nodes {
        u8coins.push(MerkleNode::into_bytevec(&node));
    }

    let leaves_vec: Vec<[u8; 32]> = u8coins
        .into_iter()
        .flat_map(|item| hash_bytes(item))
        .collect();
    return leaves_vec;
}

//loads a merkle tree from the coin leaves
pub fn load_db(coin_leaves: Vec<[u8; 32]>) -> MerkleTree<Sha256> {
    let loaded_merkle_tree = MerkleTree::<Sha256>::from_leaves(&coin_leaves);
    return loaded_merkle_tree;
}
