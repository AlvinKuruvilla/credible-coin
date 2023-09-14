use credible_coin::utils::{
    address_generator::generate_n_address_value_pairs, merkle_utils::MerkleTreeFile,
};
use rs_merkle::{algorithms::Sha256, Hasher};

fn main() {
    // generate_n_address_value_pairs(1000000);
    let merkle_file = MerkleTreeFile::new("dump.txt");
    let leaf_values = merkle_file.leaves;

    let leaves: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|x| Sha256::hash(x.as_bytes()))
        .collect();
}
