use credible_coin::{
    credible_config::get_emp_copy_path,
    emp::{
        cpp_gen::{copy_to_directory, CppFileGenerator},
        executor::{execute_compiled_binary, execute_make_install},
    },
    handle_output,
    utils::get_project_root,
};

fn main() {
    let generator = CppFileGenerator::new(&get_project_root().unwrap());
    if let Err(err) = generator.generate("gen") {
        eprintln!("Error generating C++ file: {:?}", err);
    }
    let a = copy_to_directory("gen.cpp", &get_emp_copy_path()).unwrap();
    // NOTE: If the command has 2 words, aside from sudo if "sudo_execute()" is used, the second word will
    // MUST be in the arguments array like for "make install".
    let output = execute_make_install();
    handle_output!(output);
    let output = execute_compiled_binary("bin/test_bool_gen".to_owned());
    handle_output!(output);
}
