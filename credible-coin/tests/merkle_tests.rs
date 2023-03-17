#[cfg(test)]
mod tests {
    use credible_coin::accumulator::uint_typecast::{
        u128_slice_to_byte_vector, u16_slice_to_byte_vector, u32_slice_to_byte_vector,
        u64_slice_to_byte_vector,
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
    pub fn u32_vector_test(){

        let leaf_values: Vec<u32> = vec![55,23,16,24,19,83];
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
    pub fn u16_vector_test(){

        let leaf_values: Vec<u16> = vec![55,23,16,24,19,83];
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
    pub fn u64_vector_test(){

        let leaf_values: Vec<u64> = vec![55,23,16,24,19,83];
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
    pub fn u128_vector_test(){

        let leaf_values: Vec<u128> = vec![55,23,16,24,19,83];
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
}
