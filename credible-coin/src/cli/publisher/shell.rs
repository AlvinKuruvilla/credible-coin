use reedline::{
    ColumnarMenu, DefaultCompleter, DefaultPrompt, DefaultValidator, ExampleHighlighter, Reedline,
    ReedlineMenu, Signal,
};
use rs_merkle::algorithms::Sha256 as merkle_sha;
use rs_merkle::MerkleTree;

use crate::{
    coin::Coin,
    utils::csv_utils::{addresses_and_values_as_vectors, get_address_position, update_csv_value},
};

use super::coin_map::CoinMap;

#[derive(Default)]
pub struct PublisherShell {
    tree: MerkleTree<merkle_sha>,
    filename: String,
}
pub fn shell_commands() -> Vec<String> {
    return vec![
        "exit".into(),
        "getCoinInfo".into(),
        "updateCoin".into(),
        "prove-membership".into(),
        "clear".into(),
    ];
}
/// The user is automatically brought into the publisher shell once they
/// provide a valid CSV file of their coin addresses and values and it
/// gets created into an in-memory merkle tree.
impl PublisherShell {
    pub fn new(tree: MerkleTree<merkle_sha>, filename: String) -> Self {
        return Self { tree, filename };
    }
    pub fn start(&mut self) -> std::io::Result<()> {
        //, tree: &MerkleTree<merkle_sha>
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
                            self.get_coin_info(&public_address, &self.tree);
                        } else {
                            log::error!("No public address provided for getCoinInfo");
                            break;
                        };
                    }
                    if args[0] == "updateCoin" {
                        let element = args.get(1); // Get the provided coin address and skip getCoinInfo
                        let element2 = args.get(2); // Get the new value to assign to the coin
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
                            self.tree = self.update_coin(
                                public_address,
                                value.parse().unwrap(),
                                &self.tree,
                            );
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
                        self.prove_membership(public_address, &self.tree);
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
    /// Get all of the info for a coin in the merkle tree given its public address
    fn get_coin_info(&self, _public_address: &str, tree: &MerkleTree<merkle_sha>) {
        //let tree = PublisherShell::shell_tree();
        let tree_leaves = tree
            .leaves()
            .ok_or("Could not get leaves to prove")
            .unwrap();
        let map = CoinMap::generate_address_value_map(&self.filename);
        println!(" Key Count: {:?}", map.inner.keys().len());
        //TODO: Remove unwrap
        let value = map.inner.get(_public_address).unwrap();
        let generated_coin = Coin::new(_public_address.to_owned(), *value);
        let address_index = get_address_position(&self.filename, _public_address.to_string());
        println!("Address Index:{:?}", address_index);
        println!("Address Value:{:?}", value);
        let indices = vec![address_index];
        let proof = tree.proof(&indices);
        let root = tree.root().ok_or("couldn't get the merkle root").unwrap();
        let bytes = generated_coin.serialize_coin();
        let hashed_bytes = [Coin::hash_bytes(bytes)];
        println!("Indices:{:?}", indices);
        println!("Leaf count:{:?}", tree_leaves.len());

        assert!(proof.verify(root, &indices, &hashed_bytes, tree_leaves.len()));
        log::info!("Coin Address:{:?}", _public_address);
        log::info!("Coin Value:{:?}", value);
    }
    /// Update a coin in the merkle tree given its public address and its new value
    // TODO: _new_value should be an i64 not a u32
    fn update_coin(
        &self,
        _public_address: &str,
        _new_value: u32,
        tree: &MerkleTree<merkle_sha>,
    ) -> MerkleTree<merkle_sha> {
        let tree_leaves = tree
            .leaves()
            .ok_or("Could not get leaves to prove")
            .unwrap();
        let mut map = CoinMap::generate_address_value_map(&self.filename);
        // //TODO: Remove unwrap
        let value = map.inner.get(_public_address).unwrap();
        let generated_coin = Coin::new(_public_address.to_owned(), *value);
        let address_index = get_address_position(&&self.filename, _public_address.to_string());
        let indices = vec![address_index];
        let proof = tree.proof(&indices);
        let root = tree.root().ok_or("couldn't get the merkle root").unwrap();
        let bytes = generated_coin.serialize_coin();
        let hashed_bytes = [Coin::hash_bytes(bytes)];
        assert!(proof.verify(root, &indices, &hashed_bytes, tree_leaves.len()));

        //replace value in hashmap
        let new_gen_coin = Coin::new(_public_address.to_owned(), i64::from(_new_value));
        map.replace(_public_address.to_string(), i64::from(_new_value));
        let check = map.inner.get(_public_address).unwrap();
        assert!(check == &i64::from(_new_value));
        log::info!("Coin Address:{:?}", _public_address);
        log::info!("New Coin Value:{:?}", _new_value);

        //make new merkle tree
        update_csv_value(&&self.filename, _public_address.to_owned(), i64::from(_new_value));
        let (new_addr_vec, new_val_vec) = addresses_and_values_as_vectors(&self.filename);
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
        let new_tree = MerkleTree::<merkle_sha>::from_leaves(&new_leaves);
        //TODO: Remove unwrap
        let new_address_index = get_address_position(&&self.filename, _public_address.to_string());
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

        return new_tree;
    }
    /// Prove that a coin is a member of the merkle tree given its public address
    fn prove_membership(&self, _public_address: &str, tree: &MerkleTree<merkle_sha>) {
        let tree_leaves = tree
            .leaves()
            .ok_or("Could not get leaves to prove")
            .unwrap();
        let map = CoinMap::generate_address_value_map(&self.filename);
        //TODO: Remove unwrap
        let value = map.inner.get(_public_address).unwrap();
        let generated_coin = Coin::new(_public_address.to_owned(), *value);
        let address_index = get_address_position(&&self.filename, _public_address.to_string());
        let indices = vec![address_index];
        let proof = tree.proof(&indices);
        let root = tree.root().ok_or("couldn't get the merkle root").unwrap();
        let bytes = generated_coin.serialize_coin();
        let hashed_bytes = [Coin::hash_bytes(bytes)];

        assert!(proof.verify(root, &indices, &hashed_bytes, tree_leaves.len()));
        log::info!("Address {:?} found in merkle tree", _public_address);
    }
}
