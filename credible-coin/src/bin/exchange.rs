use clap::Parser;
use credible_coin::cli::exchange::ExchangeCLI;
use eyre::Result;
pub fn main() -> Result<()> {
    return ExchangeCLI::parse().run();
}
