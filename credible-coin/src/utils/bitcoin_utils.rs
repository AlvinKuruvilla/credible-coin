use bitcoin::secp256k1::{rand, Secp256k1};
use bitcoin::{Address, Network, PublicKey};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

/// Generates a valid random Bitcoin address.
///
/// This function creates a new random key pair using the `Secp256k1` elliptic curve. It then
/// generates a Bitcoin pay-to-pubkey-hash (P2PKH) address from the public key part of the key pair.
///
/// # Returns
///
/// Returns a string representation of the generated Bitcoin address.
///
/// # Examples
///
/// ```
/// # use credible_coin::utils::bitcoin_utils::generate_address;
/// let address = generate_address();
/// assert!(address.starts_with("1") || address.starts_with("3"));
/// ```
pub fn generate_address() -> String {
    // Generate random key pair.
    let s = Secp256k1::new();
    let public_key = PublicKey::new(s.generate_keypair(&mut rand::thread_rng()).1);

    // Generate pay-to-pubkey-hash address.
    Address::p2pkh(&public_key, Network::Bitcoin).to_string()
}
/// Generates a valid Bitcoin address using the provided public key.
///
/// This function accepts a `PublicKey` and generates a Bitcoin pay-to-pubkey-hash (P2PKH)
/// address from it.
///
/// # Arguments
///
/// * `public_key` - A `PublicKey` for which the Bitcoin address needs to be generated.
///
/// # Returns
///
/// Returns a string representation of the generated Bitcoin address.
///
/// # Examples
///
/// ```
/// # use credible_coin::utils::bitcoin_utils::generate_address_with_provided_public_key;
/// # use secp256k1::Secp256k1;
///
/// let s = Secp256k1::new();
/// let public_key = s.generate_keypair(&mut rand::thread_rng()).1;
/// let address = generate_address_with_provided_public_key(public_key.into());
/// assert!(address.starts_with("1") || address.starts_with("3"));
/// ```
pub fn generate_address_with_provided_public_key(public_key: PublicKey) -> String {
    // Generate pay-to-pubkey-hash address.
    Address::p2pkh(&public_key, Network::Bitcoin).to_string()
}
/// Generates a random `n` digit number representing a Bitcoin value.
///
/// This function is useful for generating random Bitcoin transaction values, for example, when
/// simulating or testing. The generated number will have `n` digits, where `n` is provided as an
/// argument.
///
/// # Arguments
///
/// * `rng` - A mutable reference to a random number generator implementing the `rand::Rng` trait.
/// * `n` - The number of digits for the generated value.
///
/// # Returns
///
/// Returns a `u32` representing the generated `n` digit Bitcoin value.
///
/// # Examples
///
/// ```
/// # use credible_coin::utils::bitcoin_utils::generate_bitcoin_value;
/// # use rand::thread_rng;
///
/// let mut rng = thread_rng();
/// let value = generate_bitcoin_value(&mut rng, 4);
/// assert!(value >= 1000 && value < 10000);
/// ```
pub fn generate_bitcoin_value<R: rand::Rng>(rng: &mut R, n: u32) -> u32 {
    rng.gen_range(1u32..10) * 10u32.pow(n - 1) + rng.gen_range(0..10u32.pow(n - 1))
}
/// Generates `n` pairs of Bitcoin addresses and associated values.
///
/// This function produces a list of random Bitcoin addresses and their corresponding
/// transaction values. These are returned as two separate vectors - one for addresses
/// and one for their associated values.
///
/// The primary use case for this function is to simulate Bitcoin transactions for testing
/// or analysis purposes. The generated data can later be written to a CSV file or
/// processed in other ways.
///
/// # Arguments
///
/// * `n` - The number of address-value pairs to generate.
///
/// # Returns
///
/// Returns a tuple containing a vector of Bitcoin addresses (`Vec<String>`) and a
/// vector of associated values (`Vec<u32>`).
///
/// # Examples
///
/// ```
/// # use credible_coin::utils::bitcoin_utils::generate_n_address_value_pairs;
///
/// let (addresses, values) = generate_n_address_value_pairs(10);
/// assert_eq!(addresses.len(), 10);
/// assert_eq!(values.len(), 10);
/// ```
pub fn generate_n_address_value_pairs(n: u32) -> (Vec<String>, Vec<u32>) {
    let mut values = Vec::with_capacity(n as usize);
    let addresses: Vec<String> = (0..n).into_par_iter().map(|_| generate_address()).collect();
    for _ in 0..n {
        let mut rng = rand::thread_rng();
        values.push(generate_bitcoin_value(&mut rng, 6));
    }

    (addresses, values)
}
