use anyhow::Result;
use clap::{Parser, Subcommand};

/// The "Asset Database" represents the CSV file the exchange uses for
/// its secret storage
/// This module holds all of the functions to be able to create and
/// load these databases to be used within the shell and their cli
/// command implementations
pub mod asset_database;
/// A helper module to connect to a running Redis instance to
/// store exchange private keys and other sensitive data
pub mod db_connector;
/// Various helper utilities for the exchange shell
pub mod exchange_functions;
/// The core logic of the exchange shell
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
/// A wrapper type for running the exchange cli.
/// The main field is the cmd, which represent the type of command
/// to be run:
/// - Create: Create a new database csv file based on the publisher's database csv but with new addresses
/// - Load: Load the database from a csv file into a merkle tree in memory
#[derive(Debug)]
pub struct ExchangeCLI {
    #[command(subcommand)]
    cmd: ExchangeCmd,
}
impl ExchangeCLI {
    /// Runs the exchange cli and executes the command
    pub fn run(self) -> Result<()> {
        self.cmd.run()
    }
}
/// The CLI subcommand to execute:
/// - Create: Create a new database csv file based on the publisher's database csv but with new addresses
/// - Load: Load the database from a csv file into a merkle tree in memory
#[derive(Subcommand, Debug)]
pub enum ExchangeCmd {
    /// Create a new database csv file based on the publisher's database csv but with new addresses
    // TODO: This perhaps should call out to the python scripts that we made, since we have the constants for file
    //  generation pretty well defined in there
    Create(asset_database::CreateCmd),
    /// Load the database from a csv file into a merkle tree in memory
    Load(asset_database::LoadCmd),
}
impl ExchangeCmd {
    pub(crate) fn run(self) -> Result<()> {
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
        Ok(())
    }
}
