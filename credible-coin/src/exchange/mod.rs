//! The [`AbstractExchange`] trait represents common functionality all exchanges should have.

use crate::coin;
use rand;
use secp256k1;

pub trait AbstractExchange {
    //// Given an [`secp256k1::SecretKey`] private key return a [`coin::Coin`]
    /// An [`secp256k1::SecretKey`] was chosen to be as exchange agnostic as possible.
    /// It's the trait implementors job to convert the [`secp256k1::SecretKey`] to a usable format if necessary
    fn create_coin(private_key: secp256k1::SecretKey) -> coin::Coin;
    /// Given any RNG type which [`rand::Rng`] and [`Sized`] generate a private key from it.
    fn create_private_key<R: rand::Rng + ?Sized>(rng: &mut R) -> secp256k1::SecretKey;
    /// Create a [`rand::rngs::OsRng`] from a u64 seed
    fn create_random_number_generator_from_seed(seed: u64) -> rand::rngs::OsRng;
    /// Return a vector of coins
    fn create_coins(count: u32) -> Vec<coin::Coin>;
    /// Given the `path` to a proof, push it to the database
    fn push_to_db(path: String);
    /// Add a [`coin::Coin`] to the database
    fn add_coin_to_db(coin: coin::Coin);
    /// Given a [`coin::Coin`] update its value to `new_value`
    fn update_coin_value(coin: coin::Coin, new_value: u32) -> coin::Coin;
    /// Returns the internal value from the [`coin::Coin`]
    fn get_value_from_coin(coin: coin::Coin) -> u32;
}
