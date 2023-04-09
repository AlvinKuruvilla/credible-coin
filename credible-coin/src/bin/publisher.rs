use clap::Parser;
use credible_coin::cli::publisher::PublisherCLI;
use eyre::Result;
pub fn main() -> Result<()> {
    return PublisherCLI::parse().run();
}
