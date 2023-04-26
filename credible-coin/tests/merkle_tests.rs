#[cfg(test)]
mod tests {
    use credible_coin::{
        accumulator::uint_typecast::{
            u128_slice_to_byte_vector, u128_vector_to_byte_vector, u16_slice_to_byte_vector,
            u16_vector_to_byte_vector, u32_slice_to_byte_vector, u32_vector_to_byte_vector,
            u64_slice_to_byte_vector, u64_vector_to_byte_vector,
        },
        coin::Coin,
    };
    use rs_merkle::{algorithms::Sha256, Hasher, MerkleProof, MerkleTree};
    #[test]
    pub fn sanity() {
        let leaf_values = ["a", "b", "c", "d", "e", "f"];
        let leaves: Vec<[u8; 32]> = leaf_values
            .iter()
            .map(|x| Sha256::hash(x.as_bytes()))
            .collect();

        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
        let indices_to_prove = vec![3, 4];
        let leaves_to_prove = leaves.get(3..5).ok_or("can't get leaves to prove").unwrap();
        let merkle_proof = merkle_tree.proof(&indices_to_prove);
        let merkle_root = merkle_tree
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
            leaves.len()
        ));
    }
    #[test]
    pub fn u32_test() {
        let leaf_values: [u32; 6] = [12, 15, 17, 39, 34, 55];
        let leaf_bytes = u32_slice_to_byte_vector(&leaf_values);
        let leaves: Vec<[u8; 32]> = leaf_bytes.iter().map(|x| Sha256::hash(x)).collect();
        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
        let indices_to_prove = vec![3, 4];
        let leaves_to_prove = leaves.get(3..5).ok_or("can't get leaves to prove").unwrap();
        let merkle_proof = merkle_tree.proof(&indices_to_prove);
        let merkle_root = merkle_tree
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
            leaves.len()
        ));
    }
    #[test]
    pub fn u16_test() {
        let leaf_values: [u16; 6] = [87, 42, 16, 55, 73, 29];
        let leaf_bytes = u16_slice_to_byte_vector(&leaf_values);
        let leaves: Vec<[u8; 32]> = leaf_bytes.iter().map(|x| Sha256::hash(x)).collect();
        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
        let indices_to_prove = vec![3, 4];
        let leaves_to_prove = leaves.get(3..5).ok_or("can't get leaves to prove").unwrap();
        let merkle_proof = merkle_tree.proof(&indices_to_prove);
        let merkle_root = merkle_tree
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
            leaves.len()
        ));
    }

    #[test]
    pub fn u64_test() {
        let leaf_values: [u64; 6] = [12, 89, 67, 23, 58, 99];
        let leaf_bytes = u64_slice_to_byte_vector(&leaf_values);
        let leaves: Vec<[u8; 32]> = leaf_bytes.iter().map(|x| Sha256::hash(x)).collect();
        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
        let indices_to_prove = vec![3, 4];
        let leaves_to_prove = leaves.get(3..5).ok_or("can't get leaves to prove").unwrap();
        let merkle_proof = merkle_tree.proof(&indices_to_prove);
        let merkle_root = merkle_tree
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
            leaves.len()
        ));
    }

    #[test]
    pub fn u128_test() {
        let leaf_values: [u128; 6] = [12, 89, 67, 23, 58, 99];
        let leaf_bytes = u128_slice_to_byte_vector(&leaf_values);
        let leaves: Vec<[u8; 32]> = leaf_bytes.iter().map(|x| Sha256::hash(x)).collect();
        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
        let indices_to_prove = vec![3, 4];
        let leaves_to_prove = leaves.get(3..5).ok_or("can't get leaves to prove").unwrap();
        let merkle_proof = merkle_tree.proof(&indices_to_prove);
        let merkle_root = merkle_tree
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
            leaves.len()
        ));
    }

    #[test]
    pub fn u32_vector_test() {
        let leaf_values: Vec<u32> = vec![55, 23, 16, 24, 19, 83];
        let leaf_bytes = u32_vector_to_byte_vector(&leaf_values);
        let leaves: Vec<[u8; 32]> = leaf_bytes.iter().map(|x| Sha256::hash(x)).collect();
        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
        let indices_to_prove = vec![3, 4];
        let leaves_to_prove = leaves.get(3..5).ok_or("can't get leaves to prove").unwrap();
        let merkle_proof = merkle_tree.proof(&indices_to_prove);
        let merkle_root = merkle_tree
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
            leaves.len()
        ));
    }

    #[test]
    pub fn u16_vector_test() {
        let leaf_values: Vec<u16> = vec![55, 23, 16, 24, 19, 83];
        let leaf_bytes = u16_vector_to_byte_vector(&leaf_values);
        let leaves: Vec<[u8; 32]> = leaf_bytes.iter().map(|x| Sha256::hash(x)).collect();
        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
        let indices_to_prove = vec![3, 4];
        let leaves_to_prove = leaves.get(3..5).ok_or("can't get leaves to prove").unwrap();
        let merkle_proof = merkle_tree.proof(&indices_to_prove);
        let merkle_root = merkle_tree
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
            leaves.len()
        ));
    }

    #[test]
    pub fn u64_vector_test() {
        let leaf_values: Vec<u64> = vec![55, 23, 16, 24, 19, 83];
        let leaf_bytes = u64_vector_to_byte_vector(&leaf_values);
        let leaves: Vec<[u8; 32]> = leaf_bytes.iter().map(|x| Sha256::hash(x)).collect();
        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
        let indices_to_prove = vec![3, 4];
        let leaves_to_prove = leaves.get(3..5).ok_or("can't get leaves to prove").unwrap();
        let merkle_proof = merkle_tree.proof(&indices_to_prove);
        let merkle_root = merkle_tree
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
            leaves.len()
        ));
    }

    #[test]
    pub fn u128_vector_test() {
        let leaf_values: Vec<u128> = vec![55, 23, 16, 24, 19, 83];
        let leaf_bytes = u128_vector_to_byte_vector(&leaf_values);
        let leaves: Vec<[u8; 32]> = leaf_bytes.iter().map(|x| Sha256::hash(x)).collect();
        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
        let indices_to_prove = vec![3, 4];
        let leaves_to_prove = leaves.get(3..5).ok_or("can't get leaves to prove").unwrap();
        let merkle_proof = merkle_tree.proof(&indices_to_prove);
        let merkle_root = merkle_tree
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
            leaves.len()
        ));
    }
    #[test]
    pub fn bincode_serializer() {
        let leaves = [
            Sha256::hash(&bincode::serialize("a").unwrap()),
            Sha256::hash(&bincode::serialize("b").unwrap()),
            Sha256::hash(&bincode::serialize("c").unwrap()),
        ];

        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);

        let indices_to_prove = vec![0, 1];
        let leaves_to_prove = leaves.get(0..2).ok_or("can't get leaves to prove").unwrap();

        let proof = merkle_tree.proof(&indices_to_prove);
        let root = merkle_tree
            .root()
            .ok_or("couldn't get the merkle root")
            .unwrap();

        assert!(proof.verify(root, &indices_to_prove, leaves_to_prove, leaves.len()));
    }
    #[test]
    pub fn two_layer_update_test() {
        let leaves = [
            Coin::hash_bytes(Coin::new("1234".to_owned(), 123).serialize_coin()),
            Coin::hash_bytes(Coin::new("567".to_owned(), 567).serialize_coin()),
            Coin::hash_bytes(Coin::new("893".to_owned(), 111).serialize_coin()),
        ];
        let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);

        let indices_to_prove = vec![0, 1];
        let leaves_to_prove = leaves.get(0..2).ok_or("can't get leaves to prove").unwrap();

        let proof = merkle_tree.proof(&indices_to_prove);
        let root = merkle_tree
            .root()
            .ok_or("couldn't get the merkle root")
            .unwrap();

        assert!(proof.verify(root, &indices_to_prove, leaves_to_prove, leaves.len()));

        let new_leaves = [
            Coin::hash_bytes(Coin::new("901".to_owned(), 999).serialize_coin()),
            Coin::hash_bytes(Coin::new("567".to_owned(), 567).serialize_coin()),
            Coin::hash_bytes(Coin::new("893".to_owned(), 111).serialize_coin()),
        ];
        let tree = MerkleTree::<Sha256>::from_leaves(&new_leaves);

        let new_indices = vec![0, 1];
        let new_leaves_to_prove = new_leaves
            .get(0..2)
            .ok_or("can't get leaves to prove")
            .unwrap();

        let new_proof = tree.proof(&new_indices);
        let new_root = tree.root().ok_or("couldn't get the merkle root").unwrap();

        assert!(new_proof.verify(
            new_root,
            &new_indices,
            new_leaves_to_prove,
            new_leaves.len()
        ));
        assert_eq!(tree.depth(), 2);
    }
}
