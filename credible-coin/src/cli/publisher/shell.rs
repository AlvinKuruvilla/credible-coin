use flexi_logger::{AdaptiveFormat, Duplicate, FileSpec, Logger};
use nu_ansi_term::{Color, Style};
use reedline::{
    default_emacs_keybindings, ColumnarMenu, DefaultCompleter, DefaultHinter, DefaultPrompt,
    DefaultValidator, Emacs, ExampleHighlighter, KeyCode, KeyModifiers, Reedline, ReedlineEvent,
    ReedlineMenu, Signal,
};
use rs_merkle::algorithms::Sha256;
use rs_merkle::MerkleTree;

use crate::cli::publisher::publisher_functions::{cmd_table, get_coin_info, update_coin};
use crate::cli::{arg_sanitizer, convert_to_string_vec, ArgsList, CliError};
use crate::utils::merkle_utils::prove_membership;

#[derive(Default)]
pub struct PublisherShell {
    tree: MerkleTree<Sha256>,
    filename: String,
}
pub fn shell_commands() -> Vec<String> {
    return vec![
        "exit".into(),
        "getCoinInfo".into(),
        "updateCoin".into(),
        "proveMembership".into(),
        "clear".into(),
        "help".into(),
        "?".into(),
    ];
}
/// The user is automatically brought into the publisher shell once they
/// provide a valid CSV file of their coin addresses and values and it
/// gets created into an in-memory merkle tree.
impl PublisherShell {
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
            .with_hinter(Box::new(
                DefaultHinter::default().with_style(Style::new().italic().fg(Color::LightGray)),
            ))
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
                Signal::Success(buffer) => {
                    // println!("We processed: {buffer}");

                    // This is where command processing goes, see the reedline example demo for details
                    let args: Vec<&str> = buffer.split(' ').collect();
                    let args: Vec<String> = convert_to_string_vec(args);
                    if args[0] == "exit" {
                        log::info!("Exiting Shell");
                        break;
                    }
                    if args[0] == "clear" {
                        line_editor.clear_scrollback()?;
                        continue;
                    }
                    if args[0] == "getCoinInfo" {
                        arg_sanitizer::sanitize_args!(args, 1, "No public address provided");
                        // It should be safe to unwrap here because of all of the previous checking
                        let public_address = args.get(1).unwrap();
                        get_coin_info(&self.filename, public_address, &self.tree);
                    }
                    if args[0] == "updateCoin" {
                        arg_sanitizer::sanitize_args!(args, 2, "Invalid argument provided");
                        let public_address = args.get(1).unwrap();
                        if !public_address.chars().all(char::is_alphanumeric) {
                            log::error!("Invalid public address provided");
                            continue;
                        }

                        if let Some(value) = args.get(2) {
                            if let Ok(parsed_value) = value.parse::<u32>() {
                                // Perform additional operations on the parsed value if needed
                                self.tree = match update_coin(
                                    &self.filename,
                                    public_address,
                                    parsed_value,
                                    &self.tree,
                                ) {
                                    Ok(updated_tree) => updated_tree,
                                    Err(err) => {
                                        log::error!("Failed to update coin {}", err);
                                        continue;
                                    }
                                }
                            } else {
                                log::error!("Failed to parse value as a number");
                                continue;
                            }
                        } else {
                            log::error!("No new value provided");
                            continue;
                        }
                    }

                    if args[0] == "proveMembership" {
                        arg_sanitizer::sanitize_args!(args, 1, "No public address provided");
                        // It should be safe to unwrap here because of all of the previous checking
                        let public_address = args.get(1).unwrap();
                        prove_membership(&self.filename, public_address, &self.tree);
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
        println!();
        return Ok(());
    }
}
