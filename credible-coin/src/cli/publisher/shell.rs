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
                Signal::Success(buffer) => {
                    // println!("We processed: {buffer}");

                    // This is where command processing goes, see the reedline example demo for details
                    let args: Vec<&str> = buffer.split(" ").collect();
                    if args[0] == "exit" {
                        log::info!("Exiting Shell");
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
                            get_coin_info(&self.filename, &public_address, &self.tree);
                        } else {
                            log::error!("No public address provided for getCoinInfo");
                            continue;
                        };
                    }
                    if args[0] == "updateCoin" {
                        let element = args.get(1); // Get the provided coin address and skip getCoinInfo
                        let element2 = args.get(2); // Get the new value to assign to the coin
                        if let Some(public_address) = element {
                            if let Some(value) = element2 {
                                // Parse the value as a number
                                if let Ok(parsed_value) = value.parse::<u32>() {
                                    // Perform additional operations on the parsed value if needed
                                    self.tree = update_coin(
                                        &self.filename,
                                        public_address,
                                        parsed_value,
                                        &self.tree,
                                    );
                                } else {
                                    log::error!("Failed to parse value as a number");
                                    continue;
                                }
                            } else {
                                log::error!("No new value provided");
                                continue;
                            }
                        } else {
                            log::error!("No public address provided");
                            continue;
                        }
                    }
                    if args[0] == "proveMembership" {
                        if let Some(element) = args.get(1) {
                            let public_address = element;
                            prove_membership(&self.filename, public_address, &self.tree);
                        } else {
                            log::error!("No public address provided");
                            continue;
                        }
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
        Ok(())
    }
}
