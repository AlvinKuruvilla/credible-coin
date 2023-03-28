use clap::Parser;
use eyre::Result;
use credible_coin::cli::publisher::PublisherCLI;
pub fn main() -> Result<()> {
    return PublisherCLI::parse().run();
   
}