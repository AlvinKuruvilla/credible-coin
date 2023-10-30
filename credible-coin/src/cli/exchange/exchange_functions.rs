use bitcoin::PublicKey;
use comfy_table::{presets::UTF8_FULL, Attribute, Cell, ContentArrangement, Table};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rs_merkle::{algorithms::Sha256, MerkleTree};
use secp256k1::Secp256k1;

use crate::{
    cli::exchange::db_connector::insert_key_or_update, merkle_tree_entry::MerkleTreeEntry,
    utils::csv_utils::addresses_and_values_as_vectors,
};
/// Create a new SECP256K1 Private Key
pub fn create_private_key() -> PublicKey {
    let s = Secp256k1::new();
    let key = PublicKey::new(s.generate_keypair(&mut rand::thread_rng()).1);
    println!("{:?}", key.to_bytes());
    match insert_key_or_update(key.to_bytes()) {
        Ok(_) => {}
        Err(err) => log::error!("{:?}", err),
    };
    key
}
/// Create a Random Number Generator (RNG) from a provided
/// seed value
// FIXME: This function call does not save the generated RNG anywhere, but we
// should have another function responsible for that
// FIXME: We may also need to change the code so that it uses the RNG that we generate
// and give to it rather than making a thread_rng every time when generating the private key
pub fn create_rng(seed: u64) -> ChaCha8Rng {
    rand_chacha::ChaCha8Rng::seed_from_u64(seed)
}
/// Read in the csv file at the provided path and
/// construct a new Merkle Tree from it
pub fn create_new_tree_from_file(filename: &str) -> MerkleTree<Sha256> {
    let (new_addr_vec, new_val_vec) = addresses_and_values_as_vectors(filename);
    let new_vec_coin = MerkleTreeEntry::create_entries_vector(new_addr_vec, new_val_vec);
    let mut serialized_entries: Vec<Vec<u8>> = Vec::new();
    for i in new_vec_coin {
        serialized_entries.push(i.serialize_entry());
    }
    let mut new_leaves: Vec<[u8; 32]> = Vec::new();
    for u8s in serialized_entries {
        new_leaves.push(MerkleTreeEntry::hash_bytes(u8s));
    }
    MerkleTree::<Sha256>::from_leaves(&new_leaves)
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
                Cell::new("addCoinToDB").add_attribute(Attribute::Bold),
                Cell::new("Append a new coin to the CSV or SQL table given a particular value by autogenerating a new address"),
                Cell::new("Usage: `addCoinToDB <VALUE>`"),
            ]).add_row(vec![
                Cell::new("clear").add_attribute(Attribute::Bold),
                Cell::new("Clear the screen"),
                Cell::new("Usage: `clear`"),
            ]).add_row(vec![
                Cell::new("createPrivateKey").add_attribute(Attribute::Bold),
                Cell::new("Create a private key to be saved to the database"),
                Cell::new("Usage: `createPrivateKey`"),
            ]).add_row(vec![
                Cell::new("createRNG").add_attribute(Attribute::Bold),
                Cell::new("Given a seed value, create a RNG and save it to the database"),
                Cell::new("Usage: `createRNG <SEED>`"),
            ]).add_row(vec![
                Cell::new("exit").add_attribute(Attribute::Bold),
                Cell::new("Exit the shell"),
                Cell::new("Usage: `exit`"),
            ]).add_row(vec![
                Cell::new("help").add_attribute(Attribute::Bold),
                Cell::new("Print this command table"),
                Cell::new("Usage: `help`"),
            ]).add_row(vec![
                Cell::new("proveMembership").add_attribute(Attribute::Bold),
                Cell::new("Prove that the provided address is/isn't a member of the merkle tree"),
                Cell::new("Usage: `proveMembership <ADDRESS>`"),
            ]).add_row(vec![
                Cell::new("showFile").add_attribute(Attribute::Bold),
                Cell::new("Preview the file loaded into the shell"),
                Cell::new("Usage: `showFile`"),
            ]);
    println!("{table}");
}
