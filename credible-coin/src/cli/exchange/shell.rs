use crate::cli::exchange::db_connector::{insert_key_or_update, retrieve_public_key_bytes};
use crate::{
    coin::Coin,
    utils::{
        address_generator::generate_address_with_provided_public_key,
        csv_utils::{addresses_and_values_as_vectors, append_record},
        merkle_utils::prove_membership,
    },
};
use bitcoin::PublicKey;
use comfy_table::{presets::UTF8_FULL, Attribute, Cell, ContentArrangement, Table};
use nu_ansi_term::{Color, Style};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use reedline::{
    ColumnarMenu, DefaultCompleter, DefaultHinter, DefaultPrompt, DefaultValidator,
    ExampleHighlighter, Reedline, ReedlineMenu, Signal,
};
use rs_merkle::algorithms::Sha256;
use rs_merkle::MerkleTree;
use secp256k1::{rand, Secp256k1};

#[derive(Default)]
pub struct ExchangeShell {
    tree: MerkleTree<Sha256>,
    filename: String,
}
pub fn shell_commands() -> Vec<String> {
    return vec![
        "exit".into(),
        "createPrivateKey".into(),
        "proveMembership".into(),
        "addCoinToDB".into(),
        "createRNG".into(),
        "clear".into(),
        "help".into(),
        "?".into(),
    ];
}
/// The user is automatically brought into the exchange shell once they
/// provide a valid CSV file of their coin addresses and values and it
/// gets created into an in-memory merkle tree.
impl ExchangeShell {
    pub fn new(tree: MerkleTree<Sha256>, filename: String) -> Self {
        return Self { tree, filename };
    }
    pub fn start(&mut self) -> std::io::Result<()> {
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
            .with_hinter(Box::new(
                DefaultHinter::default().with_style(Style::new().italic().fg(Color::LightGray)),
            ))
            .with_ansi_colors(true);
        // Adding default menus for the compiled reedline
        line_editor = line_editor.with_menu(ReedlineMenu::EngineCompleter(Box::new(
            ColumnarMenu::default().with_name("completion_menu"),
        )));
        let prompt = DefaultPrompt::default();
        loop {
            let sig = line_editor.read_line(&prompt)?;
            match sig {
                reedline::Signal::Success(buffer) => {
                    let args: Vec<&str> = buffer.split(" ").collect();
                    if args[0] == "exit" {
                        println!("Exiting Shell");
                        break;
                    }
                    if args[0] == "clear" {
                        line_editor.clear_scrollback()?;
                        continue;
                    }
                    if args[0] == "proveMembership" {
                        let element = args.get(1); // Get the provided coin address and skip getCoinInfo
                        let public_address;
                        if element.is_some() {
                            public_address = element.unwrap();
                        } else {
                            log::error!("No public address provided");
                            break;
                        };
                        prove_membership(&self.filename, public_address, &self.tree);
                    }
                    if args[0] == "createPrivateKey" {
                        // FIXME: This function call does not save the generated private key anywhere, but we
                        // should have another function responsible for that
                        // FIXME: We may also need to change the code so that it usues the RNG that we generate
                        // and give to it rather than making a thread_rng every time
                        self.create_private_key();
                    }
                    if args[0] == "createRNG" {
                        let element = args.get(1); // Get the provided seed
                        let seed;
                        if element.is_some() {
                            seed = element.unwrap().parse::<u64>().unwrap();
                        } else {
                            log::error!("No seed provided");
                            break;
                        };
                        // FIXME: This function call does not save the generated RNG anywhere, but we
                        // should have another function responsible for that
                        // FIXME: We may also need to change the code so that it usues the RNG that we generate
                        // and give to it rather than making a thread_rng every time when generating the private key
                        self.create_rng(seed);
                    }
                    if args[0] == "addCoinToDB" {
                        let element = args.get(1); // Get the provided seed
                        let value;
                        if element.is_some() {
                            value = element.unwrap().parse::<i64>().unwrap();
                        } else {
                            log::error!("No value provided");
                            break;
                        };
                        let retrieved_bytes = retrieve_public_key_bytes();
                        if retrieved_bytes.is_empty() {
                            log::error!("Private key field not set. To set the private key call 'createPrivateKey <seed>'");
                            continue;
                        }
                        let retrieved_key: PublicKey =
                            PublicKey::from_slice(&retrieved_bytes).unwrap();
                        let address = generate_address_with_provided_public_key(retrieved_key);
                        append_record(&self.filename, address, value);
                        // FIXME: Test that the new tree is correct
                        self.tree = self.create_new_tree_from_file();
                    }
                    if args[0] == "help" || args[0] == "?" {
                        self.cmd_table();
                    }
                }
                Signal::CtrlD | Signal::CtrlC => {
                    break;
                }
            }
        }
        Ok(())
    }
    /// Create a SECP256K1 Private Key
    /// FIXME: This function call does not save the generated private key anywhere, but we
    /// should have another function responsible for that
    /// FIXME: We may also need to change the code so that it uses the RNG that we generate
    /// and give to it rather than making a thread_rng every time
    pub fn create_private_key(&self) -> PublicKey {
        let s = Secp256k1::new();
        let key = PublicKey::new(s.generate_keypair(&mut rand::thread_rng()).1);
        println!("{:?}", key.to_bytes());
        insert_key_or_update(key.to_bytes());
        return key;
    }
    /// Crreate a Random Number Generator (RNG) from a provided
    /// seed value
    /// FIXME: This function call does not save the generated RNG anywhere, but we
    /// should have another function responsible for that
    /// FIXME: We may also need to change the code so that it usues the RNG that we generate
    /// and give to it rather than making a thread_rng every time when generating the private key
    pub fn create_rng(&self, seed: u64) -> ChaCha8Rng {
        return rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    }
    /// Read in the csv file at the procided path and
    /// construct a new Merkle Tree from it
    pub fn create_new_tree_from_file(&self) -> MerkleTree<Sha256> {
        let (new_addr_vec, new_val_vec) = addresses_and_values_as_vectors(&self.filename);
        let new_vec_coin = Coin::create_coin_vector(new_addr_vec, new_val_vec);
        let mut u8coins: Vec<Vec<u8>> = Vec::new();
        for i in new_vec_coin {
            u8coins.push(i.serialize_coin());
        }
        let mut new_leaves: Vec<[u8; 32]> = Vec::new();
        for u8s in u8coins {
            new_leaves.push(Coin::hash_bytes(u8s))
        }
        let new_tree = MerkleTree::<Sha256>::from_leaves(&new_leaves);
        return new_tree;
    }
    /// The table of commands, descriptions, and usage
    pub fn cmd_table(&self) {
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
            ]);
        println!("{table}")
        //TODO: Once we actually start using sql tables for the privatekeys and RNGs
        // we should also add commands to list the available ones if they are going
        // to be selectable to be used for executing other commands like generating addresses
    }
}
