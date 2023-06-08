use bitcoin::secp256k1::{rand, Secp256k1};
use bitcoin::{Address, Network, PublicKey};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

/// Generate a valid random bitcoin address
pub fn generate_address() -> String {
    // Generate random key pair.
    let s = Secp256k1::new();
    let public_key = PublicKey::new(s.generate_keypair(&mut rand::thread_rng()).1);

    // Generate pay-to-pubkey-hash address.
    return Address::p2pkh(&public_key, Network::Bitcoin).to_string();
}
/// Generate a valid random bitcoin address using a provided public key
pub fn generate_address_with_provided_public_key(public_key: PublicKey) -> String {
    // Generate pay-to-pubkey-hash address.
    return Address::p2pkh(&public_key, Network::Bitcoin).to_string();
}
/// Generate a random n digit number for the value associated with any bitcoin address
pub fn generate_bitcoin_value<R: rand::Rng>(rng: &mut R, n: u32) -> u32 {
    return rng.gen_range(1u32..10) * 10u32.pow(n - 1) + rng.gen_range(0..10u32.pow(n - 1));
}
/// Given a number, n,  generate that many pairs of addresses and values and
/// save them to a DataFrame so they can be easily be written to a CSV later
pub fn generate_n_address_value_pairs(n: u32) -> (Vec<String>, Vec<u32>) {
    let mut values = Vec::with_capacity(n as usize);
    let addresses: Vec<String> = (0..n).into_par_iter().map(|_| generate_address()).collect();
    for _ in 0..n {
        let mut rng = rand::thread_rng();
        values.push(generate_bitcoin_value(&mut rng, 6));
    }

    (addresses, values)
}
