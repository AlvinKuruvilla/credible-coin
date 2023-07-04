use anyhow::Result;
use clap::Parser;
use credible_coin::cli::exchange::ExchangeCLI;
pub fn main() -> Result<()> {
    return ExchangeCLI::parse().run();
}
