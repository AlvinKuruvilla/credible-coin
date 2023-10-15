//! This module abstracts away all the complexity of running and parsing emp-zk
//! as a zero-knowledge backend Specifically, this module will:
//! 1. Generate cpp files which will construct a merkle tree from a provided
//! text file and try to prove membership on address at the resolved index.
//! 2. Compile and run the generated file
//! 3. Parse the output to see if the address was in the tree

//! Handles generating cpp script files on the fly using [`Template
//! Engine`](crate::emp::template_engine::TemplateEngine)
pub mod cpp_gen;
// Handles executing the cpp script file within the emp environment. This
// includes:
/// 1. Copy the file to the emp directory
/// 2. Executing make install Executing the compiled binary and parsing the
/// output
#[macro_use]
pub mod executor;
/// A simple template engine which handles dynamic ad-hoc c++ script generation
pub mod template_engine;
