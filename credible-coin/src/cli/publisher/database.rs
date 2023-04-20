use clap::Parser;

use crate::cli::publisher::shell::PublisherShell;
use crate::utils::db_funcs::{create_db, load_db, load_merkle_leaves};
use std::path::Path;

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
        create_db(&self.out_filename, self.row_count);
        //todo!();
    }
}
impl LoadCmd {
    // TODO: This needs to return an eyere::Result<()> at the end
    pub fn run(self) {
        // 1. Check if the provided csv path exists and handle errors
        assert!(Path::new(&self.filename)
            .try_exists()
            .expect("Can't find the file"));
        // 2. Try to read as dataframe and handle errors
        // 3. Try to get the data from the addresses and values columns and handle errors
        // 4. Turn into merkle tree and handle errors
        let merkle_leaves = load_merkle_leaves(&self.filename);
        let coin_tree = load_db(merkle_leaves.clone());

        print!("Provided filename: {:?}", self.filename);
        let mut publisher_shell = PublisherShell::new(coin_tree);
        publisher_shell.start();
    }
}
