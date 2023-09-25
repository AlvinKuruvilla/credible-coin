//! The [`AbstractExchange`] trait represents common functionality all exchanges should have.

use rand;
use secp256k1;

use crate::merkle_tree_entry::MerkleTreeEntry;

pub trait AbstractExchange {
    //// Given an [`secp256k1::SecretKey`] private key return a [`MerkleTreeEntry`]
    /// An [`secp256k1::SecretKey`] was chosen to be as exchange agnostic as possible.
    /// It's the trait implementors job to convert the [`secp256k1::SecretKey`] to a usable format if necessary
    fn create_coin(private_key: secp256k1::SecretKey) -> MerkleTreeEntry;
    /// Given any RNG type which [`rand::Rng`] and [`Sized`] generate a private key from it.
    fn create_private_key<R: rand::Rng + ?Sized>(rng: &mut R) -> secp256k1::SecretKey;
    /// Create a [`rand::rngs::OsRng`] from a u64 seed
    fn create_random_number_generator_from_seed(seed: u64) -> rand::rngs::OsRng;
    /// Return a vector of [`MerkleTreeEntry`]'s
    fn create_coins(count: u32) -> Vec<MerkleTreeEntry>;
    /// Given the `path` to a proof, push it to the database
    fn push_to_db(path: String);
    /// Add a [`MerkleTreeEntry`] to the database
    fn add_coin_to_db(coin: MerkleTreeEntry);
    /// Given a [`MerkleTreeEntry`] update its value to `new_value`
    fn update_entry_value(entry: MerkleTreeEntry, new_value: u32) -> MerkleTreeEntry;
    /// Returns the internal value from the [`MerkleTreeEntry`]
    fn get_value_from_entry(entry: MerkleTreeEntry) -> i64;
}
