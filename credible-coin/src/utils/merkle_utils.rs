use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};

use crate::{
    cli::publisher::coin_map::CoinMap, coin::Coin,
    utils::csv_utils::addresses_and_values_as_vectors, utils::csv_utils::get_address_position,
};

use super::nth_line_from_file;

/// Hold the parsed data from a Merkle Tree file to quickly build the tree from
pub struct MerkleTreeFile {
    pub levels: u32,
    pub num_leaves: u32,
    pub leaves: Vec<String>,
}
impl MerkleTreeFile {
    /// Construct a new `MerkleFile` from the provided filepath
    /// The file is expected to be in the following format:
    /// Line 1: Leaf count (let's call it x)
    /// Line 2: Level count
    /// Line 3-3+x-1: The leaves written in binary
    pub fn new(filepath: &str) -> Self {
        let leaf_count = nth_line_from_file(filepath, 0)
            .unwrap()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let level_count = nth_line_from_file(filepath, 1)
            .unwrap()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let start_index = 2; // 2 because at index is the level count
        let end_index = leaf_count + start_index - 1;
        let mut leaves: Vec<String> = Vec::new();
        for i in start_index..=end_index {
            let a = nth_line_from_file(filepath, i.try_into().unwrap())
                .unwrap()
                .unwrap();
            leaves.push(a);
        }
        return Self {
            levels: level_count,
            num_leaves: leaf_count,
            leaves,
        };
    }
    pub fn build_tree(hashed_leaves: Vec<String>) -> MerkleTree<Sha256> {
        //TODO: Is double hashing what we want to do?
        // The only reason we do is so that the crate will be satisfied
        // by our 256 bit numbers
        let leaves: Vec<[u8; 32]> = hashed_leaves
            .iter()
            .map(|x| Sha256::hash(x.as_bytes()))
            .collect();
        MerkleTree::<Sha256>::from_leaves(&leaves)
    }
}
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

/// Prove that a coin is a member of the merkle tree given its public address
pub fn prove_membership(filename: &str, _public_address: &str, tree: &MerkleTree<Sha256>) {
    let tree_leaves = tree
        .leaves()
        .ok_or("Could not get leaves to prove")
        .unwrap();
    let map = CoinMap::generate_address_value_map(filename);
    let value = if let Some(v) = map.inner.get(_public_address) {
        v
    } else {
        log::error!("Could not find public address {:?}", _public_address);
        return;
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
