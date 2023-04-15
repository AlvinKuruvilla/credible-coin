use credible_coin::cli::publisher::coin_map::CoinMap;
fn main() {
    let cm = CoinMap::generate_address_value_map();
    println!("Map Length: {:?}", cm.inner.keys().len())
}
