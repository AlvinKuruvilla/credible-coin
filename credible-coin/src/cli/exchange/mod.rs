use clap::{Parser, Subcommand};
use eyre::Result;

pub mod asset_database;
pub mod publisher_db_connector;
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
pub struct ExchangeCLI {
    #[command(subcommand)]
    cmd: ExchangeCmd,
}
impl ExchangeCLI {
    pub fn run(self) -> Result<()> {
        return self.cmd.run();
    }
}
#[derive(Subcommand, Debug)]
pub enum ExchangeCmd {
    /// Create a new database csv file based on the publisher's database csv but with new addresses
    Create(asset_database::CreateCmd),
    /// Load the database from a csv file into a merkle tree in memory
    Load(asset_database::LoadCmd),
}
impl ExchangeCmd {
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
