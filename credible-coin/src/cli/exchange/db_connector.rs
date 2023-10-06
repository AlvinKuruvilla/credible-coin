use anyhow::{Context, Result};
use redis::Commands;

/// A connector module responsible for creating and managing all of the information the
/// exchange needs.
///
/// The database is a redis server instance managing:
/// 1. Random Number Generators
/// 2. Private Keys
/// 3. Address-Value Pairs (TODO)
///
/// The current setup for the Random Number Generator and Private Key force there to be only
/// 1 record present at a time. If the user calls the function to generate a private key multiple
/// times, for example, the value gets overwritten every time

pub fn insert_key_or_update(key_bytes: Vec<u8>) -> Result<()> {
    let client =
        redis::Client::open("redis://127.0.0.1:6380/").context("Failed to open Redis client")?;
    let mut conn = client
        .get_connection()
        .context("Failed to establish connection to Redis")?;
    conn.set("private_key", key_bytes)
        .context("Failed to set private key")?;
    Ok(())
}
pub fn retrieve_public_key_bytes() -> Result<Vec<u8>> {
    let client =
        redis::Client::open("redis://127.0.0.1:6380/").context("Failed to open Redis client")?;
    let mut conn = client
        .get_connection()
        .context("Failed to establish connection to Redis")?;
    let key_bytes: Vec<u8> = conn
        .get("private_key")
        .context("Failed to retrieve key from Redis")?;
    Ok(key_bytes)
}

mod tests {
    use bitcoin::PublicKey;

    #[test]
    #[ignore = "Only run when connected to the redis server"]
    fn bytes_to_public_key() {
        let s = secp256k1::Secp256k1::new();
        let key = bitcoin::PublicKey::new(s.generate_keypair(&mut rand::thread_rng()).1);
        super::insert_key_or_update(key.to_bytes());
        let retrieved_bytes = super::retrieve_public_key_bytes();
        let retrieved_key: PublicKey = PublicKey::from_slice(&retrieved_bytes.unwrap()).unwrap();
        assert_eq!(key, retrieved_key);
        let client = redis::Client::open("redis://127.0.0.1:6380/").unwrap();
        // Remove the key
        let mut conn = client.get_connection().unwrap();
        redis::cmd("del").arg("private_key").execute(&mut conn);
    }
}
