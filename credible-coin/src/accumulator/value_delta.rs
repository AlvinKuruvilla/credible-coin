use super::{AbstractAccumulator, MembershipProof};
use crate::{
    credible_config::get_emp_copy_path,
    emp::{
        cpp_gen::{copy_to_directory, CppFileGenerator},
        executor::{execute_compiled_binary, execute_make_install, retrieve_membership_string},
    },
    handle_output,
    merkle_tree_entry::MerkleTreeEntry,
    utils::{
        binary_serializer::BinarySerializer, csv_utils::get_address_position, get_project_root,
    },
};
use anyhow::Result;
use std::collections::{HashMap, HashSet};
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
    fn aggregate(
        &self,
        ledger: Vec<MerkleTreeEntry>,
        exchange_entries: Vec<MerkleTreeEntry>,
    ) -> Result<i64> {
        let path: &str = "prove_member.bin";

        let membership_proof_cache: HashMap<MerkleTreeEntry, Result<MembershipProof, String>>;

        if !BinarySerializer::path_exists(path) {
            membership_proof_cache = exchange_entries
                .iter()
                .map(|entry| {
                    (
                        entry.clone(),
                        self.prove_member(entry).map_err(|e| e.to_string()),
                    )
                })
                .collect();
            BinarySerializer::serialize_to_file(&membership_proof_cache, path);
        } else {
            membership_proof_cache = BinarySerializer::deserialize_from_file(path);
        }

        // Convert the string errors back to anyhow::Error
        let membership_proof_cache = membership_proof_cache
            .into_iter()
            .map(|(k, v)| match v {
                Ok(value) => (k, Ok(value)),
                Err(err_string) => (k, Err(anyhow::anyhow!(err_string))),
            })
            .collect::<HashMap<_, Result<MembershipProof, anyhow::Error>>>();

        let delta: std::sync::Arc<std::sync::Mutex<i64>> =
            std::sync::Arc::new(std::sync::Mutex::new(0));

        ledger.iter().for_each(|ledger_entry: &MerkleTreeEntry| {
            if self.search(ledger_entry).is_ok() {
                exchange_entries.iter().for_each(|exchange_entry| {
                    let local_delta = delta.clone();
                    match membership_proof_cache.get(exchange_entry) {
                        Some(_) => {
                            let mut delta_lock = local_delta.lock().unwrap();
                            *delta_lock += exchange_entry.entry_value();
                            // println!("\x1B[32mIn the happy path!\x1B[0m");
                        }
                        None => {
                            println!("\x1B[31mFirst sad path!\x1B[0m");
                        } // Do nothing if is_member is false
                    }
                });
            }
        });

        let final_delta = *delta.lock().unwrap();
        Ok(final_delta)
    }
}
impl DeltaAccumulator {
    pub fn new(exchange_path: String) -> Self {
        return Self {
            exchange_secrets_path: exchange_path,
        };
    }
    pub fn get_all_matching_address_entries(
        &self,
        ledger_entries: Vec<MerkleTreeEntry>,
        address: String,
    ) -> Vec<MerkleTreeEntry> {
        let mut res: Vec<MerkleTreeEntry> = Vec::new();

        for entry in ledger_entries {
            if entry.entry_address() == address {
                res.push(entry);
            }
        }
        res
    }
    pub fn get_all_unique_addresses(&self, entries: Vec<MerkleTreeEntry>) -> HashSet<String> {
        let mut unique_addresses = HashSet::new();
        for entry in entries {
            unique_addresses.insert(entry.entry_address());
        }
        unique_addresses
    }
    pub fn aggregate_v2(&self, ledger: Vec<MerkleTreeEntry>) -> Result<i64> {
        let mut delta = 0;

        // Organize ledger entries by address for efficient lookups using references
        let mut ledger_by_address: std::collections::HashMap<String, Vec<&MerkleTreeEntry>> =
            std::collections::HashMap::new();
        for entry in &ledger {
            ledger_by_address
                .entry(entry.entry_address())
                .or_insert_with(Vec::new)
                .push(entry);
        }

        for addr in ledger_by_address.keys() {
            // If the address is not in exchange secrets, continue to the next
            if get_address_position(&self.exchange_secrets_path, addr.to_string(), None).is_err() {
                continue;
            }

            // If the address is valid, process its related ledger entries
            if let Some(matching_entries) = ledger_by_address.get(addr) {
                for entry_match in matching_entries {
                    match self.prove_member(entry_match) {
                        Ok(member) => {
                            if member.is_member {
                                delta += entry_match.entry_value();
                            }
                        }
                        Err(_) => {
                            println!("Could not find entry: {:?}", entry_match);
                        }
                    }
                }
            }
        }

        Ok(delta)
    }
}
