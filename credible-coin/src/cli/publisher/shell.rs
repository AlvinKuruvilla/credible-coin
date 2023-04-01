use reedline::{
    ColumnarMenu, DefaultCompleter, DefaultPrompt, DefaultValidator, ExampleHighlighter, Reedline,
    ReedlineMenu, Signal,
};

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
fn get_coin_info(public_address: &str) {
    unimplemented!()
}
/// Update a coin in the merkle tree given its public address and its new value
fn update_coin(public_address: &str, new_value: u32) {
    unimplemented!()
}
/// Prove that a coin is a member of the merkle tree given its public address
fn prove_membership(public_address: &str, value: u32) {
    unimplemented!()
}
/// The user is automatically brought into the publisher shell once they
/// provide a valid CSV file of their coin addresses and values and it
/// gets created into an in-memory merkle tree.
impl PublisherShell {
    pub fn new() -> Self {
        return Self::default();
    }
    pub fn start(&self) -> std::io::Result<()> {
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
                    if buffer.trim() == "exit" {
                        println!("Exiting Shell");
                        break;
                    }
                    if buffer.trim() == "clear" {
                        line_editor.clear_scrollback()?;
                        continue;
                    }
                    if buffer.trim() == "getCoinInfo" {
                        let args: Vec<&str> = buffer.split(" ").collect();
                        let element = args.get(2); // Get the provided coin address and skip getCoinInfo
                        if let Some(public_address) = element {
                            get_coin_info(&public_address);
                        } else {
                            log::error!("No public address provided");
                            break;
                        };
                    }
                    if buffer.trim() == "updateCoin" {
                        let args: Vec<&str> = buffer.split(" ").collect();
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
                            update_coin(public_address, value.parse().unwrap());
                        } else {
                            log::error!("No new value provided");
                            break;
                        }
                    }
                    if buffer.trim() == "prove-membership" {
                        let args: Vec<&str> = buffer.split(" ").collect();
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
                            prove_membership(public_address, value.parse().unwrap());
                        } else {
                            log::error!("No new value provided");
                            break;
                        }
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