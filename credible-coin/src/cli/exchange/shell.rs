use crate::cli::exchange::db_connector::retrieve_public_key_bytes;
use crate::cli::exchange::exchange_functions::{
    cmd_table, create_new_tree_from_file, create_private_key, create_rng,
};
use crate::cli::{arg_sanitizer, convert_to_string_vec, ArgsList, CliError};
use crate::utils::{
    address_generator::generate_address_with_provided_public_key, csv_utils::append_record,
    merkle_utils::prove_membership,
};
use bitcoin::PublicKey;
use flexi_logger::{AdaptiveFormat, Duplicate, FileSpec, Logger};
use nu_ansi_term::Color;
use reedline::{
    default_emacs_keybindings, ColumnarMenu, DefaultCompleter, DefaultHinter, DefaultPrompt,
    DefaultValidator, Emacs, ExampleHighlighter, KeyCode, KeyModifiers, Reedline, ReedlineEvent,
    ReedlineMenu, Signal,
};
use rs_merkle::algorithms::Sha256;
use rs_merkle::MerkleTree;

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
    pub fn start(&mut self) -> anyhow::Result<()> {
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
                    let args: Vec<String> = convert_to_string_vec(args);
                    if args[0] == "exit" {
                        log::info!("Exiting Shell");
                        break;
                    }
                    if args[0] == "clear" {
                        line_editor.clear_scrollback()?;
                        continue;
                    }
                    if args[0] == "proveMembership" {
                        arg_sanitizer::sanitize_args!(args, 1, "No public address provided");
                        // It should be safe to unwrap here because of all of the previous checking
                        let public_address = args.get(1).unwrap();
                        println!("Public address{:?}", public_address);
                        prove_membership(&self.filename, public_address, &self.tree);
                    }
                    if args[0] == "createPrivateKey" {
                        create_private_key();
                    }
                    if args[0] == "createRNG" {
                        arg_sanitizer::sanitize_args!(args, 1, "No seed provided");
                        // It is safe to do unwrap the get() here because, sanitize_args! ensures that the value is not empty,
                        // but we still need a match case for parsing the value to a string
                        let seed = match args.get(1).unwrap().parse::<u64>() {
                            Ok(value) => value,
                            Err(_) => {
                                log::error!("Invalid seed provided");
                                continue;
                            }
                        };

                        // FIXME: This function call does not save the generated RNG anywhere, but we
                        // should have another function responsible for that
                        // FIXME: We may also need to change the code so that it uses the RNG that we generate
                        // and give to it rather than making a thread_rng every time when generating the private key
                        create_rng(seed);
                    }
                    if args[0] == "addCoinToDB" {
                        arg_sanitizer::sanitize_args!(args, 1, "No value provided");
                        // It is safe to do unwrap the get() here because, sanitize_args! ensures that the value is not empty,
                        // but we still need a match case for parsing the value to a string
                        let value = match args.get(1).unwrap().parse::<u64>() {
                            Ok(value) => value,
                            Err(_) => {
                                log::error!("Invalid value provided");
                                continue;
                            }
                        };
                        let mut retrieved_bytes: Vec<u8> = Vec::default();
                        match retrieve_public_key_bytes() {
                            Ok(key_bytes) => {
                                retrieved_bytes = key_bytes;
                            }
                            Err(err) => {
                                log::error!("{:?}", err);
                                continue;
                            }
                        };

                        if retrieved_bytes.is_empty() {
                            log::error!("Private key field not set. To set the private key call 'createPrivateKey <seed>'");
                            continue;
                        }
                        let retrieved_key: PublicKey =
                            PublicKey::from_slice(&retrieved_bytes).unwrap();
                        let address = generate_address_with_provided_public_key(retrieved_key);
                        append_record(&self.filename, address, value);
                        // FIXME: Test that the new tree is correct
                        self.tree = create_new_tree_from_file(&self.filename);
                    }
                    if args[0] == "help" || args[0] == "?" {
                        cmd_table();
                    }
                }
                Signal::CtrlD | Signal::CtrlC => {
                    break;
                }
            }
        }
        return Ok(());
    }
}
