use anyhow::Result;
use credible_coin::accumulator::{value_delta::DeltaAccumulator, AbstractAccumulator};
use std::env;
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    // Remember arg[0] is the name of the executable
    let (v1, v2): (Vec<String>, Vec<i64>) =
        credible_coin::utils::csv_utils::addresses_and_values_as_vectors(&args[1]);
    let publisher_set: Vec<credible_coin::merkle_tree_entry::MerkleTreeEntry> =
        credible_coin::merkle_tree_entry::MerkleTreeEntry::create_entries_vector(v1, v2);
    let a = credible_coin::utils::csv_utils::get_exchange_addresses_and_values_from_file(&args[2]);
    let exchange_set = credible_coin::utils::csv_utils::into_merkle_tree_entries(a);
    let d = DeltaAccumulator::new(args[1].clone());
    let res = d.aggregate(publisher_set, exchange_set)?;
    println!("{}", res);
    Ok(())
}
