#[cfg(test)]
mod tests {
    use credible_coin::accumulator::merkle::{
        hash_concat, hash_data, verify_merkle_proof, Data, MerkleTree,
    };

    // Helper function to verify Merkle proofs for each data leaf in the tree
    fn verify_merkle_proofs(data: &Vec<Vec<u8>>, tree: &MerkleTree) {
        for data_leaf in data {
            let proof = tree
                .get_merkle_proof_by_data(&data_leaf)
                .expect("Should be able to create proof");

            assert!(verify_merkle_proof(&proof, &data_leaf, &tree.root_hash()));
        }
    }

    #[test]
    fn two_level_tree() {
        let data = vec![Data::from("A"), Data::from("B")];

        assert!(MerkleTree::verify(
            &data,
            &hash_concat(&hash_data(&data[0]), &hash_data(&data[1]))
        ));

        let tree = MerkleTree::construct(&data);

        assert_eq!(tree.levels, 2);
        assert_eq!(tree.num_leaves(), 2);
        assert_eq!(tree.nodes.len(), 3);
        assert_eq!(tree.leaves().len(), 2);
    }

    #[test]
    fn three_level_tree() {
        let data = vec![
            Data::from("AAA"),
            Data::from("BBB"),
            Data::from("CCC"),
            Data::from("DDD"),
        ];

        let expected_hash = hash_concat(
            &hash_concat(&hash_data(&data[0]), &hash_data(&data[1])),
            &hash_concat(&hash_data(&data[2]), &hash_data(&data[3])),
        );

        assert!(MerkleTree::verify(&data, &expected_hash));

        let tree = MerkleTree::construct(&data);

        assert_eq!(tree.levels, 3);
        assert_eq!(tree.num_leaves(), 4);
        assert_eq!(tree.nodes.len(), 7);
        assert_eq!(tree.leaves().len(), 4);
    }

    #[test]
    fn four_level_tree() {
        let data = vec![
            Data::from("AAAA"),
            Data::from("BBBB"),
            Data::from("CCCC"),
            Data::from("DDDD"),
            Data::from("EEEE"),
            Data::from("FFFF"),
            Data::from("GGGG"),
            Data::from("HHHH"),
        ];

        let expected_hash = hash_concat(
            &hash_concat(
                &hash_concat(&hash_data(&data[0]), &hash_data(&data[1])),
                &hash_concat(&hash_data(&data[2]), &hash_data(&data[3])),
            ),
            &hash_concat(
                &hash_concat(&hash_data(&data[4]), &hash_data(&data[5])),
                &hash_concat(&hash_data(&data[6]), &hash_data(&data[7])),
            ),
        );

        assert!(MerkleTree::verify(&data, &expected_hash));

        let tree = MerkleTree::construct(&data);

        assert_eq!(tree.levels, 4);
        assert_eq!(tree.num_leaves(), 8);
        assert_eq!(tree.nodes.len(), 15);
        assert_eq!(tree.leaves().len(), 8);
    }

    #[test]
    fn verify_merkle_proof_two_layer() {
        let data = vec![Data::from("AAAA"), Data::from("BBBB")];

        let tree = MerkleTree::construct(&data);

        verify_merkle_proofs(&data, &tree);
    }

    #[test]
    fn verify_merkle_proof_larger() {
        let data = vec![
            Data::from("AAAA"),
            Data::from("BBBB"),
            Data::from("CCCC"),
            Data::from("DDDD"),
        ];

        let tree = MerkleTree::construct(&data);

        verify_merkle_proofs(&data, &tree);
    }

    #[test]
    fn verify_merkle_tree_middle_node() {
        let data = vec![
            Data::from("AAAA"),
            Data::from("BBBB"),
            Data::from("CCCC"),
            Data::from("DDDD"),
        ];

        let tree = MerkleTree::construct(&data);

        verify_merkle_proofs(&data, &tree);
    }

    #[test]
    fn verify_merkle_tree_8() {
        let data = vec![
            Data::from("AAAA"),
            Data::from("BBBB"),
            Data::from("CCCC"),
            Data::from("DDDD"),
            Data::from("EEEE"),
            Data::from("FFFF"),
            Data::from("GGGG"),
            Data::from("HHHH"),
        ];

        let tree = MerkleTree::construct(&data);

        verify_merkle_proofs(&data, &tree);
    }

    #[test]
    fn verify_merkle_tree_16() {
        let data = vec![
            Data::from("AAAA"),
            Data::from("BBBB"),
            Data::from("CCCC"),
            Data::from("DDDD"),
            Data::from("EEEE"),
            Data::from("FFFF"),
            Data::from("GGGG"),
            Data::from("HHHH"),
            Data::from("IIII"),
            Data::from("JJJJ"),
            Data::from("KKKK"),
            Data::from("LLLL"),
            Data::from("MMMM"),
            Data::from("NNNN"),
            Data::from("OOOO"),
            Data::from("PPPP"),
        ];

        let tree = MerkleTree::construct(&data);

        verify_merkle_proofs(&data, &tree);
    }

    #[test]
    fn merkle_proof_fails_for_wrong_data() {
        let data = vec![Data::from("AAAA"), Data::from("BBBB")];

        let tree = MerkleTree::construct(&data);
        let proof = tree
            .get_merkle_proof_by_data(&Data::from("BBBB"))
            .expect("Should be able to create proof");
        assert!(!verify_merkle_proof(
            &proof,
            &Data::from("AAAA"),
            &tree.root_hash()
        ));
    }

    #[test]
    fn merkle_proof_fails_for_wrong_tree() {
        let data = vec![Data::from("AAAA"), Data::from("BBBB")];

        let tree = MerkleTree::construct(&data);

        let other_data = vec![
            Data::from("AAAA"),
            Data::from("BBBB"),
            Data::from("CCCC"),
            Data::from("DDDD"),
        ];

        let other_tree = MerkleTree::construct(&other_data);

        let proof = tree
            .get_merkle_proof_by_data(&Data::from("AAAA"))
            .expect("Should be able to create proof");
        assert!(!verify_merkle_proof(
            &proof,
            &Data::from("AAAA"),
            &other_tree.root_hash()
        ));
    }

    #[test]
    fn merkle_proof_fails_if_tree_changed() {
        let data = vec![Data::from("AAAA"), Data::from("BBBB")];

        let tree = MerkleTree::construct(&data);

        let other_data = vec![Data::from("AAAA"), Data::from("BBBA")];

        let other_tree = MerkleTree::construct(&other_data);

        let proof = tree
            .get_merkle_proof_by_data(&Data::from("AAAA"))
            .expect("Should be able to create proof");

        assert!(!verify_merkle_proof(
            &proof,
            &Data::from("AAAA"),
            &other_tree.root_hash()
        ));
    }

    #[test]
    fn test_merkle_proof_fails_for_invalid_index() {
        let data = vec![Data::from("AAAA"), Data::from("BBBB")];

        let tree = MerkleTree::construct(&data);

        assert!(tree.get_merkle_proof_by_index(3).is_err());
    }
}
