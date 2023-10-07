use std::collections::HashMap;

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
pub struct DeltaAccumulator {
    pub exchange_secrets_path: String,
}
impl AbstractAccumulator for DeltaAccumulator {
    fn prove_member(&self, element: MerkleTreeEntry) -> Result<MembershipProof> {
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
        let s = retrieve_membership_string(output)?;
        if s == "leaf does have path to root" {
            Ok(MembershipProof { is_member: true })
        } else {
            Ok(MembershipProof { is_member: false })
        }
    }

    fn verify(&self, element_proof: MembershipProof) {
        todo!()
    }

    fn search(&self, entry: MerkleTreeEntry) -> Result<usize> {
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
        let mut delta: i64 = 0;
        for ledger_entry in ledger.iter() {
            match self.search(ledger_entry.clone()) {
                Ok(_) => {
                    for exchange_entry in exchange_entries.iter() {
                        match self.prove_member(exchange_entry.clone()) {
                            Ok(_) => delta += exchange_entry.entry_value(),
                            Err(_) => continue,
                        }
                    }
                }
                Err(_) => continue,
            }
        }
        Ok(delta)
    }
}
