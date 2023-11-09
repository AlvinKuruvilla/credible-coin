use anyhow::Result;
use clap::{Parser, Subcommand};
/// The publisher database represents a CSV representation of the data
/// a cryptocurrency provider would provide to the exchange
/// This module holds all of the functions to be able to create and
/// load these databases to be used within the shell and their cli
/// command implementations
pub mod database;
/// A map of address to value pairs
pub mod entry_map;
/// Various helper utilities for the publisher shell
pub mod publisher_functions;
/// The core logic of the publisher shell
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
/// A wrapper type for running the publisher cli.
/// The main field is the cmd, which represent the type of command
/// to be run:
/// - Create: Create a new database csv file from our test data
/// - Load: Load the database from a csv file into a merkle tree in memory
#[derive(Debug)]
pub struct PublisherCLI {
    #[command(subcommand)]
    cmd: PublisherCmd,
}
impl PublisherCLI {
    /// Run the cli
    pub fn run(self) -> Result<()> {
        self.cmd.run()
    }
}
/// The CLI subcommand to execute:
/// - Create: Create a new database csv file from our test data
/// - Load: Load the database from a csv file into a merkle tree in memory
#[derive(Subcommand, Debug)]
pub enum PublisherCmd {
    /// Save the database into a csv file
    Create(database::CreateCmd),
    /// Load the database from a csv file and load as a merkle tree into memory
    Load(database::LoadCmd),
}
impl PublisherCmd {
    pub(crate) fn run(self) -> Result<()> {
        // So the usage would look like `publisher load <ARGS>` or `publisher create <ARGS>`
        // So we have 2 potential commands to run: load or create, now we just have to parse the arguments
        match self {
            Self::Create(cmd) => {
                cmd.run()?;
            }
            Self::Load(cmd) => {
                cmd.run()?;
            }
        }
        Ok(())
    }
}
