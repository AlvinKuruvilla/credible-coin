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
///
/// # Returns
///
/// * `Ok()`: Successfully inserted or updated the public key.
/// * `Err(anyhow::Error)`: An error context containing the reason for failure, e.g., connection issues, key not found, etc.
///
/// # Panics
///
/// The function may panic if the Redis server is not running or if there are other unforeseen issues.

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
/// Retrieves the public key bytes from a Redis instance.
///
/// This function connects to a Redis instance at the provided URI and fetches the bytes associated with the key "private_key".
/// Note: The function name suggests retrieving a public key, but the Redis key is named "private_key". Ensure that the correct key is being fetched.
///
/// # Returns
///
/// * `Ok(Vec<u8>)`: A vector of bytes representing the public key.
/// * `Err(anyhow::Error)`: An error context containing the reason for failure, e.g., connection issues, key not found, etc.
///
/// # Panics
///
/// The function may panic if the Redis server is not running or if there are other unforeseen issues.
///
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
        let _ = super::insert_key_or_update(key.to_bytes());
        let retrieved_bytes = super::retrieve_public_key_bytes();
        let retrieved_key: PublicKey = PublicKey::from_slice(&retrieved_bytes.unwrap()).unwrap();
        assert_eq!(key, retrieved_key);
        let client = redis::Client::open("redis://127.0.0.1:6380/").unwrap();
        // Remove the key
        let mut conn = client.get_connection().unwrap();
        redis::cmd("del").arg("private_key").execute(&mut conn);
    }
}
