use rusqlite::Connection;

const DB_URL: &str = "sqlite://exchange.db";

/// A connector module responsible for creating and managing all of the databases and tables the publisher needs.
///
/// The `globsl_publisher.db` database file manages the following tables:
/// 1. Random Number Generators
/// 2. Private Keys
/// 3. Adddress-Value Pairs (TODO)
///
/// The current setup for the Random Number Generator and Private Key tables force there to be only 1 record
/// present at a time. If the user calls the function to generate a private key multiple times, for example,
/// The first record of the table will be overwritten every time

/// Create all the tables if they do not exist already
pub fn init_tables() {
    let conn = Connection::open(DB_URL).unwrap();
    // NOTE: sqlite essentially already provides an autoincrementing integer primary key
    // through row_id so we don't need to make a seperate column for it
    conn.execute("CREATE TABLE IF NOT EXISTS rng (rng_bytes BLOB);", ())
        .unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS private_keys (private_key_bytes BLOB);",
        (),
    )
    .unwrap();
}
pub fn insert_key_or_update(key_bytes: Vec<u8>) {
    let conn = Connection::open(DB_URL).unwrap();
    conn.execute(
        "INSERT OR REPLACE INTO private_keys (private_key_bytes) VALUES (?1)",
        [key_bytes
            .iter()
            .cloned()
            .map(|element| element.to_string())
            .collect::<String>()],
    )
    .unwrap();
}
pub fn retrieve_key_bytes() {
    let conn = Connection::open(DB_URL).unwrap();
    conn.execute(
        "SELECT * FROM private_keys ORDER BY ROWID ASC LIMIT 1
    ",
        (),
    )
    .unwrap();
}
