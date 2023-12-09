use super::{AbstractAccumulator, MembershipProof};
use crate::{
    credible_config::get_emp_copy_path,
    emp::{
        cpp_gen::{copy_to_directory, CppFileGenerator},
        executor::{execute_compiled_binary, execute_make_install, retrieve_membership_string},
    },
    handle_status,
    merkle_tree_entry::MerkleTreeEntry,
    utils::{csv_utils::get_address_position, get_project_root},
};
use anyhow::Result;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
/// In cryptographic protocols, an accumulator is a primitive that allows you to
/// represent a set of elements and prove membership (or non-membership) without
/// revealing which elements are in the set A Delta Accumulator is a variation
/// on that idea but having a secret set of address and value pairs representing
/// the exchange's secret set Then using the public ledger, the accumulator runs
/// a membership on each of its entries finding all of the entries the exchange
/// can prove membership of and gets their values This value becomes the value
/// of the exchange's liabilities
#[derive(Debug)]
pub struct DeltaAccumulator {
    /// The path to the exchange secrets file
    pub exchange_secrets_path: String,
}
impl AbstractAccumulator for DeltaAccumulator {
    fn prove_member(
        &self,
        element: &MerkleTreeEntry,
        pos: Option<usize>,
    ) -> Result<MembershipProof> {
        let mut sub_map: HashMap<String, String> = HashMap::new();
        if pos.is_none() {
            let pos = self.search(element)?;
            sub_map.insert("actual_leaf_index".to_string(), pos.to_string());
        }
        println!("pos: {:?}", pos);
        // crate::_pause();
        sub_map.insert("actual_leaf_index".to_string(), pos.unwrap().to_string());
        let generator = CppFileGenerator::new(&get_project_root().unwrap(), sub_map);
        if let Err(err) = generator.generate("gen") {
            eprintln!("Error generating C++ file: {:?}", err);
        }
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let _ = copy_to_directory("gen.cpp", &get_emp_copy_path()).await;
        });
        let output = execute_make_install();
        handle_status!(output);
        let output = execute_compiled_binary("bin/test_bool_gen".to_owned());
        let s = retrieve_membership_string(output)?;
        println!("{:?}", s);
        if s == "leaf does have path to root" {
            Ok(MembershipProof { is_member: true })
        } else {
            Ok(MembershipProof { is_member: false })
        }
    }

    fn verify(&self, _element_proof: MembershipProof) {
        panic!("This function should not be called");
    }
    fn search(&self, entry: &MerkleTreeEntry) -> Result<usize> {
        println!(
            "{:?}, {:?}: {:?}",
            entry.entry_address(),
            entry.entry_value(),
            get_address_position(&self.exchange_secrets_path, entry.entry_address(), None)?
        );
        Ok(get_address_position(
            &self.exchange_secrets_path,
            entry.entry_address(),
            None,
        )?)
    }
    fn aggregate(&self, ledger_file: String, ledger_entries: Vec<MerkleTreeEntry>) -> Result<i64> {
        // Precompute the matching entries for each unique address
        let matching_entries_map = self.precompute_matching_entries(&ledger_entries);

        let mut delta = 0;
        let reset = "\x1b[0m";
        let green = "\x1b[32m";

        let all_matching_entries = matching_entries_map.values().flatten();
        let mut entry_index = 0;
        for entry_match in all_matching_entries {
            println!(
                "Index {}{}{}: of: {}{}{}",
                green,
                entry_index,
                reset,
                green,
                matching_entries_map.values().flatten().count(),
                reset
            );
            let pos = get_address_position(
                &ledger_file,
                entry_match.entry_address(),
                Some(entry_match.entry_value()),
            )?;
            println!("pos: {:?}", pos);
            println!("Entry Match: {:?}", entry_match);
            // crate::_pause();
            //*TODO: Double check that the +1 ensures that the C++ merkle tree
            //* picks the correct entry to prove membership
            //*
            //* NOTE: The +1 is very important to ensure that we account
            //* for the lack of headers in the out.txt file that the c++ merkle tree
            //* uses. This is particularly important because the entry it would try to prove
            //* otherwise would be right above the one we actually want to prove
            match self.prove_member(&entry_match, Some(pos + 1)) {
                Ok(member_proof) if member_proof.is_member => {
                    println!(
                        "Current delta: {:?} + value: {:?}",
                        delta,
                        entry_match.entry_value()
                    );
                    delta += entry_match.entry_value();
                }
                Ok(_) => {
                    println!(
                        "Entry is not member  {:?}: {:?}",
                        entry_match.entry_address(),
                        entry_match.entry_value()
                    );
                }
                Err(_) => println!("Failed to prove member {}", entry_match),
            }
            entry_index += 1;
        }

        Ok(delta)
    }
}
impl DeltaAccumulator {
    /// Make a new `DeltaAccumulator` from the provided exchange_path string
    pub fn new(exchange_path: String) -> Self {
        return Self {
            exchange_secrets_path: exchange_path.into(),
        };
    }
    /// Returns all `MerkleTreeEntry` items from the provided ledger entries that match the specified address.
    ///
    /// # Arguments
    ///
    /// * `ledger_entries` - A slice containing ledger entries to search through.
    /// * `address` - The target address to match against the entries.
    ///
    pub fn get_all_matching_address_entries(
        &self,
        ledger_entries: &[MerkleTreeEntry],
        address: &str,
    ) -> Vec<MerkleTreeEntry> {
        ledger_entries
            .par_iter()
            .filter(|entry| entry.entry_address() == address)
            .cloned()
            .collect()
    }
    /// Precomputes and groups ledger entries by their address.
    ///
    /// This function constructs a HashMap where the keys are addresses and the values are vectors of `MerkleTreeEntry` items
    /// that share the same address. This allows for efficient retrieval of all entries associated with a particular address.
    ///
    /// # Arguments
    ///
    /// * `ledger_entries` - A slice containing ledger entries to be grouped by address.
    ///
    /// # Returns
    ///
    /// A `HashMap` where each key is an address string, and each value is a vector of `MerkleTreeEntry` items associated with that address.
    pub fn precompute_matching_entries(
        &self,
        ledger_entries: &[MerkleTreeEntry],
    ) -> HashMap<String, Vec<MerkleTreeEntry>> {
        let mut map: HashMap<String, Vec<MerkleTreeEntry>> = HashMap::new();

        for entry in ledger_entries {
            map.entry(entry.entry_address())
                .or_insert_with(Vec::new)
                .push(entry.clone());
        }

        map
    }
}
