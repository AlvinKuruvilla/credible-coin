use anyhow::Result;
use credible_coin::accumulator::{value_delta::DeltaAccumulator, AbstractAccumulator};
use std::env;
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    // Remember arg[0] is the name of the executable
    if !credible_coin::emp::executor::is_ccache_installed() {
        println!("ccache is not installed.");
        return Ok(());
    }

    let (v1, v2): (Vec<String>, Vec<i64>) =
        credible_coin::utils::csv_utils::addresses_and_values_as_vectors(&args[1]);
    let publisher_set: Vec<credible_coin::merkle_tree_entry::MerkleTreeEntry> =
        credible_coin::merkle_tree_entry::MerkleTreeEntry::create_entries_vector(v1, v2);
    let d = DeltaAccumulator::new(args[1].clone());
    let res = d.aggregate(args[1].clone(), publisher_set)?;
    println!("{}", res);
    Ok(())
}
