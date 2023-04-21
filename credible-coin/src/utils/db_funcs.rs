use crate::coin::*;
use crate::utils::address_generator::generate_n_address_value_pairs;
use crate::utils::csv_utils::*;
use csv::Writer;
use rs_merkle::{algorithms::Sha256, MerkleTree};
use std::path::Path;

/// Creates csv file from random addresses and values
pub fn create_db(filename: &str, row_count: u32) {
    assert!(
        !Path::new(filename)
            .try_exists()
            .expect("file already exists"),
        "file already exists"
    );
    let (addresses, values )= generate_n_address_value_pairs(row_count);
    // Create the file but don't save the handle
    std::fs::File::create(filename).unwrap();
    let mut writer = Writer::from_path(filename).unwrap();
    assert_eq!(addresses.len(), values.len());
    writer.write_record(&["transaction_hash","block_hash","block_number","block_timestamp","index","script_asm","script_hex","required_signatures","hash_type","addresses","value"]).unwrap();
    
    for (index, address) in addresses.iter().enumerate() {
        // Since our CSVRecord struct has more field than just addresses and values
        // we need to have garbage data when we are writing a new file so if we ever read 
        // the file again we donn't crash because of mismatched feilds
        writer.write_record(&["0", "0", "0", "0", "0", "0", "0", "0", "0", address, &values[index].to_string()]).unwrap();
    }
    writer.flush().unwrap();
}

/// Creates leaves from coin vectors
pub fn load_merkle_leaves(file_name: &str) -> Vec<[u8; 32]> {
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
        leaves_vec.push(Coin::hash_bytes(coin))
    }
    return leaves_vec;
}

// Loads a merkle tree from the coin leaves
pub fn load_db(coin_leaves: Vec<[u8; 32]>) -> MerkleTree<Sha256> {
    let loaded_merkle_tree = MerkleTree::<Sha256>::from_leaves(&coin_leaves);
    return loaded_merkle_tree;
}
