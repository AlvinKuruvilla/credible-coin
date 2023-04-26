//! This module is reponsible for creating the CLI and shell functionality of each component of our system.
//! The `bin/` folder then has an associated binary program for each of the components
//!
//! We have 4 executable program components  
//! 1. Publisher: The publisher acts like a psuedo cryptocurrency exchange responsible for pulling and
//! modifying address and value data from the blockchain. From there it can take this data and
//! load it into a Merkle tree for the exchange to use later to generate proofs
//! 2. Exchange: The exchange represents the company that a customer would store their assets on.
//! The exchange's main job is to communicate with the verifier compnent to generate solvency
//! proofs, and can perform various functions to that end. (See the module docs for more details)
//! 3. Verifier: Similar to the Exchange, the Verifier's main function is to send
//! solvency requests and manage the generated proofs accordingly. (See the module docs for more details)
//! 4. Customer: The customer's job is to take the generated proofs and run its own checks on them
//! utilizing our CLI/shell to make a more informed decision about the safety and security of
//! their assets  (See the module docs for more details)
pub mod exchange;
pub mod publisher;
