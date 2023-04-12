use reedline::{
    ColumnarMenu, DefaultCompleter, DefaultPrompt, DefaultValidator, ExampleHighlighter, Reedline,
    ReedlineMenu, Signal,
};
use rs_merkle::algorithms::Sha256 as merkle_sha;
use rs_merkle::MerkleTree;

use crate::{
    coin::Coin,
    merkle::{hash_bytes, MerkleNode},
    utils::csv_utils::get_address_position,
};

use super::coin_map::CoinMap;

#[derive(Default)]
pub struct PublisherShell;
pub fn shell_commands() -> Vec<String> {
    return vec![
        "exit".into(),
        "getCoinInfo".into(),
        "updateCoin".into(),
        "prove-membership".into(),
        "clear".into(),
    ];
}
/// Get all of the info for a coin in the merkle tree given its public address
fn get_coin_info(_public_address: &str, tree: &MerkleTree<merkle_sha>) {
    let tree_leaves = tree
        .leaves()
        .ok_or("Could not get leaves to prove")
        .unwrap();
    let map = CoinMap::generate_address_value_map();
    //TODO: Remove unwrap
    let value = map.inner.get(_public_address).unwrap();
    let generated_coin = Coin::new(_public_address.to_owned(), *value);
    let address_index = get_address_position(_public_address.to_string());
    let indices = vec![address_index];
    let proof = tree.proof(&indices);
    let root = tree.root().ok_or("couldn't get the merkle root").unwrap();
    let node = MerkleNode::new(generated_coin);
    let bytes = MerkleNode::into_bytevec(&node);
    let hashed_bytes = [hash_bytes(bytes)];
    assert!(proof.verify(root, &indices, &hashed_bytes, tree_leaves.len()));
    log::info!("Coin Address:{:?}", _public_address);
    log::info!("Coin Value:{:?}", value);
}
/// Update a coin in the merkle tree given its public address and its new value
fn update_coin(_public_address: &str, _new_value: u32, tree: &MerkleTree<merkle_sha>) {
    //unimplemented!()
    let tree_leaves = tree
        .leaves()
        .ok_or("Could not get leaves to prove")
        .unwrap();
    let mut map = CoinMap::generate_address_value_map();
    //TODO: Remove unwrap
    let value = map.inner.get(_public_address).unwrap();
    let generated_coin = Coin::new(_public_address.to_owned(), *value);
    let address_index = get_address_position(_public_address.to_string());
    let indices = vec![address_index];
    let proof = tree.proof(&indices);
    let root = tree.root().ok_or("couldn't get the merkle root").unwrap();
    let node = MerkleNode::new(generated_coin);
    let bytes = MerkleNode::into_bytevec(&node);
    let hashed_bytes = [hash_bytes(bytes)];
    assert!(proof.verify(root, &indices, &hashed_bytes, tree_leaves.len()));
    
    //replace value in hashmap
    let new_gen_coin = Coin::new(_public_address.to_owned(), i64::from(_new_value));
    map.replace(_public_address.to_string(), i64::from(_new_value));
    let check = map.inner.get(_public_address).unwrap();
    assert!(check == &i64::from(_new_value));
    log::info!("Coin Address:{:?}", _public_address);
    log::info!("New Coin Value:{:?}", _new_value);
    /*
    //replace value in merkle tree
    let new_node = MerkleNode::new(new_gen_coin);
    let new_bytes = MerkleNode::into_bytevec(&new_node);
    let new_hashed_bytes = [hash_bytes(new_bytes)];
    let mut vec_nodes: Vec<MerkleNode> = tree_leaves.to_vec();
    vec_nodes[address_index] = new_hashed_bytes;
    return MerkleTree::<merkle_sha>::from_leaves(vec_nodes);
    */
}
/// Prove that a coin is a member of the merkle tree given its public address
fn prove_membership(_public_address: &str, tree: &MerkleTree<merkle_sha>) {
    let tree_leaves = tree
        .leaves()
        .ok_or("Could not get leaves to prove")
        .unwrap();
    let map = CoinMap::generate_address_value_map();
    //TODO: Remove unwrap
    let value = map.inner.get(_public_address).unwrap();
    let generated_coin = Coin::new(_public_address.to_owned(), *value);
    let address_index = get_address_position(_public_address.to_string());
    let indices = vec![address_index];
    let proof = tree.proof(&indices);
    let root = tree.root().ok_or("couldn't get the merkle root").unwrap();
    let node = MerkleNode::new(generated_coin);
    let bytes = MerkleNode::into_bytevec(&node);
    let hashed_bytes = [hash_bytes(bytes)];
    assert!(proof.verify(root, &indices, &hashed_bytes, tree_leaves.len()));
    log::info!("Address {:?} found in merkle tree", _public_address);
}
/// The user is automatically brought into the publisher shell once they
/// provide a valid CSV file of their coin addresses and values and it
/// gets created into an in-memory merkle tree.
impl PublisherShell {
    pub fn new() -> Self {
        return Self::default();
    }
    pub fn start(&self, tree: &MerkleTree<merkle_sha>) -> std::io::Result<()> {
        println!("Ctrl-D or Ctrl-C to quit");
        pretty_env_logger::init();
        let commands = shell_commands();
        let completer = Box::new(DefaultCompleter::new_with_wordlen(commands.clone(), 2));
        let mut line_editor = Reedline::create()
            .with_highlighter(Box::new(ExampleHighlighter::new(commands)))
            .with_completer(completer)
            .with_quick_completions(true)
            .with_partial_completions(true)
            .with_validator(Box::new(DefaultValidator))
            .with_ansi_colors(true);
        // Adding default menus for the compiled reedline
        line_editor = line_editor.with_menu(ReedlineMenu::EngineCompleter(Box::new(
            ColumnarMenu::default().with_name("completion_menu"),
        )));
        let prompt = DefaultPrompt::default();

        loop {
            let sig = line_editor.read_line(&prompt)?;
            match sig {
                Signal::Success(buffer) => {
                    println!("We processed: {buffer}");
                    // This is where command processing goes, see the reedline example demo for details
                    let args: Vec<&str> = buffer.split(" ").collect();
                    if args[0] == "exit" {
                        println!("Exiting Shell");
                        break;
                    }
                    if args[0] == "clear" {
                        line_editor.clear_scrollback()?;
                        continue;
                    }
                    if args[0] == "getCoinInfo" {
                        let element = args.get(1); // Get the provided coin address and skip getCoinInfo
                        println!("Provided public address {:?}", element);
                        if let Some(public_address) = element {
                            get_coin_info(&public_address, tree);
                        } else {
                            log::error!("No public address provided for getCoinInfo");
                            break;
                        };
                    }
                    // TODO: change the if buffer trim statements to be similar to getCoinInfo
                    if args[0] == "updateCoin" {
                        let element = args.get(2); // Get the provided coin address and skip getCoinInfo
                        let element2 = args.get(3); // Get the new value to assign to the coin
                        let public_address;
                        if element.is_some() {
                            public_address = element.unwrap();
                        } else {
                            log::error!("No public address provided");
                            break;
                        };
                        if element2.is_some() {
                            let value = element2.unwrap();
                            // TODO: We should do some math or 'if let Some' magic for the value in case we cannot parse it
                            update_coin(public_address, value.parse().unwrap(), tree);
                        } else {
                            log::error!("No new value provided");
                            break;
                        }
                    }
                    if args[0] == "prove-membership" {
                        let element = args.get(1); // Get the provided coin address and skip getCoinInfo
                        let public_address;
                        if element.is_some() {
                            public_address = element.unwrap();
                        } else {
                            log::error!("No public address provided");
                            break;
                        };
                        prove_membership(public_address, tree);
                    }
                }
                Signal::CtrlD | Signal::CtrlC => {
                    println!("\nAborted!");
                    break;
                }
            }
        }
        println!();
        Ok(())
    }
}
