//! Benchmark binary: exercises exactly one prove_member call.
//!
//! On master: template render → copy → make install → ./run → parse stdout
//! On emp-rs-integration: load entries → build tree → prove membership (FFI)
//!
//! Usage: bench_proof

use credible_coin::accumulator::{value_delta::DeltaAccumulator, AbstractAccumulator};
use credible_coin::merkle_tree_entry::MerkleTreeEntry;

fn main() {
    let d = DeltaAccumulator::new("dummy".to_string());
    let entry = MerkleTreeEntry::new(
        "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
        5000000000,
    );

    match d.prove_member(&entry, Some(0)) {
        Ok(proof) => {
            if proof.is_member() {
                println!("PASS: leaf is a member");
            } else {
                println!("FAIL: leaf is not a member");
            }
        }
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
            std::process::exit(1);
        }
    }
}
