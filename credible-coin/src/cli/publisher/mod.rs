use clap::{Parser, Subcommand};
use eyre::Result;

pub mod coin_map;
mod database;
mod shell;

const VERSION: &str = "0.0.1";

static HELP_TEMPLATE: &str = "\
{before-help}{name} {version}
{author}
{about}
{usage-heading}
  {usage}
{all-args}{after-help}";

#[derive(Parser)]
#[command(
    author = "Alvin Kuruvilla, Nilsso Diaz",
    version = VERSION,
    help_template(HELP_TEMPLATE),
)]
pub struct PublisherCLI {
    #[command(subcommand)]
    cmd: PublisherCmd,
}
impl PublisherCLI {
    pub fn run(self) -> Result<()> {
        return self.cmd.run();
    }
}
#[derive(Subcommand, Debug)]
pub enum PublisherCmd {
    /// Save the database into a csv file
    Create(database::CreateCmd),
    /// Load the database from a csv file and load as a merkle tree into memory
    Load(database::LoadCmd),
}
impl PublisherCmd {
    pub fn run(self) -> Result<()> {
        // So the usage would look like `publisher load <ARGS>` or `publisher create <ARGS>`
        // So we have 2 potential commands to run: load or create, now we just have to parse the arguments
        match self {
            Self::Create(cmd) => {
                cmd.run();
            }
            Self::Load(cmd) => {
                cmd.run();
            }
        }
        return Ok(());
    }
}
