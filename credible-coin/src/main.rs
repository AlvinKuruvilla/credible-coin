use credible_coin::{
    credible_config::get_emp_path,
    emp::cpp_gen::{copy_to_directory, CppFileGenerator},
    utils::{get_project_root, merkle_utils::MerkleTreeFile},
};
use rs_merkle::{algorithms::Sha256, Hasher};

fn main() {
    // generate_n_address_value_pairs(1000000);
    let merkle_file = MerkleTreeFile::new("dump.txt");
    let leaf_values = merkle_file.leaves;

    let leaves: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|x| Sha256::hash(x.as_bytes()))
        .collect();
    let generator = CppFileGenerator::new(&get_project_root().unwrap());
    if let Err(err) = generator.generate("gen") {
        eprintln!("Error generating C++ file: {:?}", err);
    }
    let a = copy_to_directory("gen.cpp", &get_emp_path()).unwrap();
    // KEEP: Example of how we can hopefully ask for sudo permissions
    // while the program is running
    // https://users.rust-lang.org/t/how-to-execute-a-root-command-on-linux/50066/9
    //
    // TODO: Check if this works in the shell environment as well
    // assert!(::std::process::Command::new("sudo")
    //     .arg("/usr/bin/id")
    //     .status()
    //     .unwrap()
    //     .success());
}
