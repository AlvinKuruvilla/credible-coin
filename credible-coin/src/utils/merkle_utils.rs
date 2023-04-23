use crate::coin::Coin;

use super::csv_utils::{
    addresses_and_values_as_vectors, get_exchange_addresses_and_values_from_file,
};

/// Creates leaves from coin vectors
pub fn load_merkle_leaves_for_publisher(file_name: &str) -> Vec<[u8; 32]> {
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
pub fn load_merkle_leaves_for_exchange(file_name: &str) -> Vec<[u8; 32]> {
    let (v1, v2) = get_exchange_addresses_and_values_from_file(file_name);
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
