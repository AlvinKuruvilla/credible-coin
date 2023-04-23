use crate::utils::{
    address_generator::generate_address, csv_utils::append_record, merkle_utils::prove_membership,
};
use bitcoin::PublicKey;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use reedline::{
    ColumnarMenu, DefaultCompleter, DefaultPrompt, DefaultValidator, ExampleHighlighter, Reedline,
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
                        // and give to it rather than making a thread_rng every time
                        self.createRNG(seed);
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
                        let address = generate_address();
                        append_record(&self.filename, address, value)
                        // FIXME: We have to make a new tree here and assign it to the old one like we did for the publisher
                    }
                }
                Signal::CtrlD | Signal::CtrlC => {
                    break;
                }
            }
        }
        Ok(())
    }
    pub fn create_private_key(&self) -> PublicKey {
        let s = Secp256k1::new();
        return PublicKey::new(s.generate_keypair(&mut rand::thread_rng()).1);
    }
    pub fn createRNG(&self, seed: u64) -> ChaCha8Rng {
        return rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    }
}
