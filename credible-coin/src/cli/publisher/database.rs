use clap::Parser;

use crate::cli::publisher::shell::PublisherShell;

#[derive(Parser, Debug)]
#[command(infer_subcommands = true)]
pub struct CreateCmd {
    out_filename: String,
    row_count: u32,
}
#[derive(Parser, Debug)]
#[command(infer_subcommands = true)]
pub struct LoadCmd {
    filename: String,
}
impl CreateCmd {
    // TODO: This needs to return an eyere::Result<()> at the end
    pub fn run(self) {
        // 1. Check that the outfile doesn't already exist and handle errors
        // 2. IDK
        todo!();
    }
}
impl LoadCmd {
    // TODO: This needs to return an eyere::Result<()> at the end
    pub fn run(self) {
        // 1. Check if the provided csv path exists and handle errors
        // 2. Try to read as dataframe and handle errors
        // 3. Try to get the data from the addresses and values columns and handle errors
        // 4. Turn into merkle tree and handle errors
        print!("Provided filename: {:?}", self.filename);
        let publisher_shell = PublisherShell::new();
        publisher_shell.start();
    }
}
