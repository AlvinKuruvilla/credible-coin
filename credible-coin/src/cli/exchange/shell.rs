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
use flexi_logger::{AdaptiveFormat, Duplicate, FileSpec, Logger};
use nu_ansi_term::Color;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use reedline::{
    default_emacs_keybindings, ColumnarMenu, DefaultCompleter, DefaultHinter, DefaultPrompt,
    DefaultValidator, Emacs, ExampleHighlighter, KeyCode, KeyModifiers, Reedline, ReedlineEvent,
    ReedlineMenu, Signal,
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
        let commands = shell_commands();
        let completer: Box<DefaultCompleter> =
            Box::new(DefaultCompleter::new_with_wordlen(commands.clone(), 2));
        // Use the interactive menu to select options from the completer
        let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu"));
        // Set up the required keybindings
        let mut keybindings = default_emacs_keybindings();
        keybindings.add_binding(
            KeyModifiers::NONE,
            KeyCode::Tab,
            ReedlineEvent::UntilFound(vec![
                ReedlineEvent::Menu("completion_menu".to_string()),
                ReedlineEvent::MenuNext,
            ]),
        );

        let edit_mode = Box::new(Emacs::new(keybindings));

        let mut line_editor = Reedline::create()
            .with_highlighter(Box::new(ExampleHighlighter::new(commands)))
            .with_completer(completer)
            .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
            .with_ansi_colors(true)
            .with_quick_completions(true)
            .with_partial_completions(true)
            .with_hinter(Box::new(DefaultHinter::default().with_style(
                nu_ansi_term::Style::new().italic().fg(Color::LightGray),
            )))
            .with_validator(Box::new(DefaultValidator))
            .with_edit_mode(edit_mode);
        let prompt = DefaultPrompt::default();
        //TODO: Eventually swap WriteMode::Default with WriteMode::Async
        Logger::try_with_str("info")
            .expect("Could not create logger object")
            .duplicate_to_stderr(Duplicate::Warn)
            .duplicate_to_stdout(Duplicate::All)
            .log_to_file(
                FileSpec::default()
                    .basename("credible")
                    .suffix("log")
                    .suppress_timestamp(),
            )
            .adaptive_format_for_stderr(AdaptiveFormat::Default)
            .adaptive_format_for_stdout(AdaptiveFormat::Default)
            .append()
            .start()
            .unwrap();
        loop {
            let sig = line_editor.read_line(&prompt)?;
            match sig {
                reedline::Signal::Success(buffer) => {
                    let args: Vec<&str> = buffer.split(" ").collect();
                    if args[0] == "exit" {
                        log::info!("Exiting Shell");
                        break;
                    }
                    if args[0] == "clear" {
                        line_editor.clear_scrollback()?;
                        continue;
                    }
                    if args[0] == "proveMembership" {
                        match args.get(1) {
                            Some(element) => {
                                let public_address = element;
                                if public_address.is_empty() {
                                    log::error!("Empty public address provided");
                                    continue;
                                }
                                prove_membership(&self.filename, public_address, &self.tree);
                            }
                            None => {
                                log::error!("No public address provided");
                                continue;
                            }
                        }
                    }
                    if args[0] == "createPrivateKey" {
                        self.create_private_key();
                    }
                    if args[0] == "createRNG" {
                        let seed = match args.get(1) {
                            Some(element) => match element.parse::<u64>() {
                                Ok(value) => value,
                                Err(_) => {
                                    log::error!("Invalid seed provided");
                                    continue;
                                }
                            },
                            None => {
                                log::error!("No seed provided");
                                continue;
                            }
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
    }
}
