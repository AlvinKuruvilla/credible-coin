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
        sub_map.insert("actual_leaf_index".to_string(), pos.unwrap().to_string());
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
        // TODO: Replace with match
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
                        self.prove_member(entry, None).map_err(|e| e.to_string()),
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
    /// Compute the delta accumulation for the given ledger file and set of ledger entries
    pub fn aggregate_v2(&self, ledger_file: String, ledger: Vec<MerkleTreeEntry>) -> Result<i64> {
        // Precompute the matching entries for each unique address
        let matching_entries_map = self.precompute_matching_entries(&ledger);

        let unique_addresses = matching_entries_map.keys();
        let mut delta = 0;

        for addr in unique_addresses {
            if let Some(matching_entries) = matching_entries_map.get(addr) {
                // println!("{:?}", matching_entries);
                for entry_match in matching_entries {
                    // println!("Current entry: {:?}", entry_match);
                    let pos = get_address_position(
                        &ledger_file,
                        entry_match.entry_address(),
                        Some(entry_match.entry_value()),
                    )?;
                    match self.prove_member(&entry_match, Some(pos)) {
                        Ok(member_proof) => match member_proof.is_member {
                            true => {
                                println!(
                                    "Current delta: {:?} + value: {:?}",
                                    delta,
                                    entry_match.entry_value()
                                );
                                delta += entry_match.entry_value();
                            }
                            false => {
                                println!(
                                    "Entry is not member  {:?}: {:?}",
                                    entry_match.entry_address(),
                                    entry_match.entry_value()
                                );
                            }
                        },
                        Err(_) => println!("Failed to prove member {}", entry_match),
                    }
                }
            }
        }
        Ok(delta)
    }
}
