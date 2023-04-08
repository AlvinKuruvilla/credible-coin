#[cfg(test)]
mod tests{
	use credible_coin::utils::db_func::*;
	use std::path::Path;
	
	//tests the createDB function
	#[test]
	pub fn create_db_test(){
		createDB();
		assert!(Path::new("test.csv").try_exists().expect("Can't find the file"));
	}

	//tests load_merkle_leaves and loadDB
	#[test]
	pub fn load_db_test(){
		let merkle_coin_leaves = load_merkle_leaves();
		let merkle_with_coins = loadDB(merkle_coin_leaves.clone());

        	let indices_to_prove = vec![3, 4];
        	let leaves_to_prove = merkle_coin_leaves.get(3..5).ok_or("can't get leaves to prove").unwrap();
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