use anyhow::Result;
use clap::Parser;
use credible_coin::cli::publisher::PublisherCLI;
pub fn main() -> Result<()> {
    return PublisherCLI::parse().run();
}
