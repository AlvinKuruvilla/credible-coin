use credible_coin::cli::publisher::coin_map::CoinMap;
fn main() {
    let cm = CoinMap::generate_address_value_map("BigQuery Bitcoin Historical Data - outputs.csv");
    println!("Map Length: {:?}", cm.inner.keys().len())
}
