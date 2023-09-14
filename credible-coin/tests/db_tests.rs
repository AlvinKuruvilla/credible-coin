#[cfg(test)]
mod tests {
    use credible_coin::cli::exchange::asset_database::create_exchange_database;
    use credible_coin::cli::publisher::database::{create_db, load_db};
    use credible_coin::utils::merkle_utils::load_merkle_leaves_from_csv;
    use rs_merkle::{algorithms::Sha256, MerkleProof};
    use std::fs;
    use std::path::Path;

    //tests the create_db function
    #[test]
    pub fn create_db_test() {
        // TODO: This should return a result with an error variant if the file already exists
        create_db("test.csv", 20);
        assert!(Path::new("test.csv")
            .try_exists()
            .expect("Can't find the file"));
        fs::remove_file("test.csv").expect("Could not delete file");
    }
    #[test]
    pub fn repeat_publisher_db_create_test() {
        for _ in 0..1000 {
            create_db("pub_test.csv", 20);
            Path::new("pub_test.csv")
                .try_exists()
                .expect("Can't find the file");
            fs::remove_file("pub_test.csv").expect("Could not delete file");
        }
    }

    //tests load_merkle_leaves and load_DB for both the publisher and the exchange, since they should have the exact same code flow
    #[test]
    pub fn load_db_test() {
        let merkle_coin_leaves =
            load_merkle_leaves_from_csv("BigQuery Bitcoin Historical Data - outputs.csv");
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
    #[test]
    pub fn create_exchange_db_test() {
        create_exchange_database(
            "BigQuery Bitcoin Historical Data - outputs.csv",
            "test.csv",
            20,
        );
        assert!(Path::new("test.csv")
            .try_exists()
            .expect("Can't find the file"));
        fs::remove_file("test.csv").expect("Could not delete file");
    }
    #[test]
    pub fn repeat_exchange_db_create() {
        for _ in 0..1000 {
            create_exchange_database(
                "BigQuery Bitcoin Historical Data - outputs.csv",
                "test.csv",
                20,
            );
            Path::new("test.csv")
                .try_exists()
                .expect("Can't find the file");
            fs::remove_file("test.csv").expect("Could not delete file");
        }
    }
    #[test]
    pub fn string_bytes() {
        let a = 123;
        let numeric_bytes = bincode::serialize(&a).unwrap();
        let s = match std::str::from_utf8(&numeric_bytes) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        assert_eq!(s.as_bytes(), numeric_bytes);
        let b: i32 = bincode::deserialize(s.as_bytes()).unwrap();
        assert_eq!(a, b);
    }
}
