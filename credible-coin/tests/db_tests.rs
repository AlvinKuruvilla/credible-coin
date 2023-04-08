#[cfg(test)]
mod tests {
    use credible_coin::utils::db_funcs::{create_db, load_db, load_merkle_leaves};
    use rs_merkle::{algorithms::Sha256, MerkleProof};
    use std::fs;
    use std::path::Path;

    //tests the createDB function
    #[test]
    pub fn create_db_test() {
        create_db();
        assert!(Path::new("test.csv")
            .try_exists()
            .expect("Can't find the file"));
        fs::remove_file("test.csv").expect("Could not delete file");
    }

    //tests load_merkle_leaves and load_DB
    #[test]
    pub fn load_db_test() {
        let merkle_coin_leaves = load_merkle_leaves();
        let merkle_with_coins = load_db(merkle_coin_leaves.clone());

        let indices_to_prove = vec![3, 4];
        let leaves_to_prove = merkle_coin_leaves
            .get(3..5)
            .ok_or("can't get leaves to prove")
            .unwrap();
        let merkle_proof = merkle_with_coins.proof(&indices_to_prove);
        let merkle_root = merkle_with_coins
            .root()
            .ok_or("couldn't get the merkle root")
            .unwrap();
        // Serialize proof to pass it to the client
        let proof_bytes = merkle_proof.to_bytes();

        // Parse proof back on the client
        let proof = MerkleProof::<Sha256>::try_from(proof_bytes).unwrap();

        assert!(proof.verify(
            merkle_root,
            &indices_to_prove,
            leaves_to_prove,
            merkle_coin_leaves.len()
        ));
    }
}
