use anyhow::{anyhow, Result};
use comfy_table::{presets::UTF8_FULL, Attribute, Cell, ContentArrangement, Table};
use rs_merkle::{algorithms::Sha256, MerkleTree};

use crate::{
    cli::publisher::coin_map::CoinMap,
    coin::Coin,
    utils::csv_utils::{addresses_and_values_as_vectors, get_address_position, update_csv_value},
};

/// Get all of the info for a coin in the merkle tree given its public address
pub fn get_coin_info(filename: &str, public_address: &str, tree: &MerkleTree<Sha256>) {
    //let tree = PublisherShell::shell_tree();
    let tree_leaves = tree
        .leaves()
        .ok_or("Could not get leaves to prove")
        .unwrap();
    let map = CoinMap::generate_address_value_map(filename);
    let value = match map.inner.get(public_address) {
        Some(v) => v,
        None => {
            log::error!("Could not find public address {:?}", public_address);
            return;
        }
    };
    let generated_coin = Coin::new(public_address.to_owned(), *value);
    let address_index = get_address_position(filename, public_address.to_string());
    // println!("Address Index:{:?}", address_index);
    // println!("Address Value:{:?}", value);
    let indices = vec![address_index];
    let proof = tree.proof(&indices);
    let root = tree.root().ok_or("couldn't get the merkle root").unwrap();
    let bytes = generated_coin.serialize_coin();
    let hashed_bytes = [Coin::hash_bytes(bytes)];
    println!("Indices:{:?}", indices);
    println!("Leaf count:{:?}", tree_leaves.len());

    assert!(proof.verify(root, &indices, &hashed_bytes, tree_leaves.len()));
    println!("Coin Address:{:?}", public_address);
    println!("Coin Value:{:?}", value);
}
/// Update a coin in the merkle tree given its public address and its new value
// TODO: _new_value should be an i64 not a u32
pub fn update_coin(
    filename: &str,
    _public_address: &str,
    _new_value: u32,
    tree: &MerkleTree<Sha256>,
) -> Result<MerkleTree<Sha256>> {
    let tree_leaves = tree
        .leaves()
        .ok_or("Could not get leaves to prove")
        .unwrap();
    let mut map = CoinMap::generate_address_value_map(filename);
    let value = match map.inner.get(_public_address) {
        Some(v) => v,
        None => {
            return Err(anyhow!("Public address not found"));
        }
    };
    let generated_coin = Coin::new(_public_address.to_owned(), *value);
    let address_index = get_address_position(filename, _public_address.to_string());
    let indices = vec![address_index];
    let proof = tree.proof(&indices);
    let root = tree.root().ok_or("couldn't get the merkle root").unwrap();
    let bytes = generated_coin.serialize_coin();
    let hashed_bytes = [Coin::hash_bytes(bytes)];
    assert!(proof.verify(root, &indices, &hashed_bytes, tree_leaves.len()));

    //replace value in hashmap
    let new_gen_coin = Coin::new(_public_address.to_owned(), i64::from(_new_value));
    map.replace(_public_address.to_string(), i64::from(_new_value));
    let check = match map.inner.get(_public_address) {
        Some(v) => v,
        None => {
            return Err(anyhow!("Public address not found"));
        }
    };
    assert!(check == &i64::from(_new_value));
    println!("Coin Address:{:?}", _public_address);
    println!("New Coin Value:{:?}", _new_value);

    //make new merkle tree
    update_csv_value(filename, _public_address.to_owned(), i64::from(_new_value));
    let (new_addr_vec, new_val_vec) = addresses_and_values_as_vectors(filename);
    assert!(new_val_vec.contains(&i64::from(_new_value)));
    let new_vec_coin = Coin::create_coin_vector(new_addr_vec, new_val_vec);
    // println!("_______________________________________________________");
    // for c in new_vec_coin.iter() {
    //     println!("Bytes= {:?}", c.serialize_coin());
    // }
    let mut u8coins: Vec<Vec<u8>> = Vec::new();
    for i in new_vec_coin {
        u8coins.push(i.serialize_coin());
    }
    // println!("{:?}", u8coins);
    // std::thread::sleep(std::time::Duration::from_millis(100000));
    // println!("*************************************************************");

    let mut new_leaves: Vec<[u8; 32]> = Vec::new();
    for u8s in u8coins {
        new_leaves.push(Coin::hash_bytes(u8s))
    }
    let new_tree = MerkleTree::<Sha256>::from_leaves(&new_leaves);
    //TODO: Remove unwrap
    let new_address_index = get_address_position(&&filename, _public_address.to_string());
    let new_indices = vec![new_address_index];
    let new_proof = new_tree.proof(&new_indices);
    let new_root = new_tree
        .root()
        .ok_or("couldn't get the merkle root")
        .unwrap();
    println!(
        "{:?} {:?}",
        new_gen_coin.coin_address(),
        new_gen_coin.coin_value()
    );
    let new_bytes = new_gen_coin.serialize_coin();
    let new_hashed_bytes = [Coin::hash_bytes(new_bytes)];
    assert_ne!(new_tree.root(), tree.root());
    // assert_ne!(new_hashed_bytes, hashed_bytes);

    assert!(new_proof.verify(new_root, &new_indices, &new_hashed_bytes, new_leaves.len()));

    return Ok(new_tree);
}
/// The table of commands, descriptions, and usage
pub fn cmd_table() {
    let mut table = Table::new();
    table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(80)
            .set_header(vec![
                Cell::new("Command").add_attribute(Attribute::Bold),
                Cell::new("Description").add_attribute(Attribute::Bold),
                Cell::new("Usage").add_attribute(Attribute::Bold),
            ])
            .add_row(vec![
                Cell::new("?").add_attribute(Attribute::Bold),
                Cell::new("Print this command table"),
                Cell::new("Usage: `?`"),
            ])
            .add_row(vec![
                Cell::new("clear").add_attribute(Attribute::Bold),
                Cell::new("Clear the screen"),
                Cell::new("Usage: `clear`"),
            ])
            .add_row(vec![
                Cell::new("exit").add_attribute(Attribute::Bold),
                Cell::new("Exit the shell"),
                Cell::new("Usage: `exit`"),
            ])
            .add_row(vec![
                Cell::new("getCoinInfo").add_attribute(Attribute::Bold),
                Cell::new("Given an address, if the address is present in the CSV return basic information about it"),
                Cell::new("Usage: `getCoinInfo <ADDRESS>`"),
            ])
            .add_row(vec![
                Cell::new("help").add_attribute(Attribute::Bold),
                Cell::new("Print this command table"),
                Cell::new("Usage: `help`"),
            ])
            .add_row(vec![
                Cell::new("proveMembership").add_attribute(Attribute::Bold),
                Cell::new("Prove that the provided address is/isn't a member of the merkle tree"),
                Cell::new("Usage: `proveMembership <ADDRESS>`"),
            ]).add_row(vec![
                Cell::new("updateCoin").add_attribute(Attribute::Bold),
                Cell::new("Given an address, if the address is present in the CSV, update its value with the provided value"),
                Cell::new("Usage: `updateCoin <ADDRESS> <NEW VALUE>`"),
            ]);
    println!("{table}")
}
