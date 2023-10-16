use super::{AbstractAccumulator, MembershipProof};
use crate::{
    credible_config::get_emp_copy_path,
    emp::{
        cpp_gen::{copy_to_directory, CppFileGenerator},
        executor::{execute_compiled_binary, execute_make_install, retrieve_membership_string},
    },
    handle_output,
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
pub struct DeltaAccumulator {
    pub exchange_secrets_path: String,
}
impl AbstractAccumulator for DeltaAccumulator {
    fn prove_member(&self, element: &MerkleTreeEntry) -> Result<MembershipProof> {
        let pos = self.search(element)?;
        let mut sub_map: HashMap<String, String> = HashMap::new();
        sub_map.insert("actual_leaf_index".to_string(), pos.to_string());
        let generator = CppFileGenerator::new(&get_project_root().unwrap(), sub_map);
        if let Err(err) = generator.generate("gen") {
            eprintln!("Error generating C++ file: {:?}", err);
        }
        let a = copy_to_directory("gen.cpp", &get_emp_copy_path()).unwrap();
        let output = execute_make_install();
        handle_output!(output);
        let output = execute_compiled_binary("bin/test_bool_gen".to_owned());
        // println!("{:?}", output);
        let s = retrieve_membership_string(output)?;
        if s == "leaf does have path to root" {
            Ok(MembershipProof { is_member: true })
        } else {
            Ok(MembershipProof { is_member: false })
        }
    }

    fn verify(&self, element_proof: MembershipProof) {
        panic!("This function should not be called");
    }
    fn search(&self, entry: &MerkleTreeEntry) -> Result<usize> {
        Ok(get_address_position(
            &self.exchange_secrets_path,
            entry.entry_address(),
            None,
        )?)
    }
    fn aggregate(
        &self,
        ledger: Vec<MerkleTreeEntry>,
        exchange_entries: Vec<MerkleTreeEntry>,
    ) -> Result<i64> {
        // Create a HashMap to cache the results of prove_member for each exchange_entry
        let membership_proof_cache: HashMap<_, _> = exchange_entries
            .iter()
            .map(|entry| (entry.clone(), self.prove_member(entry)))
            .collect();

        let delta = std::sync::Arc::new(std::sync::Mutex::new(0));

        ledger.par_iter().for_each(|ledger_entry| {
            if self.search(ledger_entry).is_ok() {
                exchange_entries.par_iter().for_each(|exchange_entry| {
                    if let Some(prove_member_result) = membership_proof_cache.get(exchange_entry) {
                        if prove_member_result.is_ok() {
                            let mut delta_lock = delta.lock().unwrap();
                            *delta_lock += exchange_entry.entry_value();
                        }
                    } else {
                        unreachable!()
                    }
                });
            }
        });

        Ok(*delta.clone().lock().unwrap())
    }
}
impl DeltaAccumulator {
    pub fn new(exchange_path: String) -> Self {
        return Self {
            exchange_secrets_path: exchange_path,
        };
    }
}
