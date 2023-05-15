use bitcoin::PublicKey;
use redis::Commands;

/// A connector module responsible for creating and managing all of the databases and tables the
/// publisher needs.
///
/// The `exchange.db` database file manages the following tables:
/// 1. Random Number Generators
/// 2. Private Keys
/// 3. Adddress-Value Pairs (TODO)
///
/// The current setup for the Random Number Generator and Private Key tables force there to be only
/// 1 record present at a time. If the user calls the function to generate a private key multiple
/// times, for example, The first record of the table will be overwritten every time

pub fn insert_key_or_update(key_bytes: Vec<u8>) {
    let client = redis::Client::open("redis://127.0.0.1:6380/").unwrap();
    let mut conn = client.get_connection().unwrap();
    // TODO: Panic here
    // let byte_string = std::str::from_utf8(&key_bytes).unwrap().to_owned();
    let _: () = conn.set("private_key", key_bytes).unwrap();
}
pub fn retrieve_public_key() -> bitcoin::PublicKey {
    let client = redis::Client::open("redis://127.0.0.1:6380/").unwrap();
    let mut conn = client.get_connection().unwrap();
    let key_bytes: Vec<u8> = conn.get("private_key").unwrap();

    return PublicKey::from_slice(&key_bytes).unwrap();
}

mod tests {

    #[test]
    fn bytes_to_public_key() {
        let s = secp256k1::Secp256k1::new();
        let key = bitcoin::PublicKey::new(s.generate_keypair(&mut rand::thread_rng()).1);
        super::insert_key_or_update(key.to_bytes());
        let retrieved_key = super::retrieve_public_key();
        assert_eq!(key, retrieved_key);
        let client = redis::Client::open("redis://127.0.0.1:6380/").unwrap();
        // Remove the key
        let mut conn = client.get_connection().unwrap();
        redis::cmd("del").arg("private_key").execute(&mut conn);
    }
}
