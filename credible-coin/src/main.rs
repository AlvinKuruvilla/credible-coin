use credible_coin::utils;
fn main() {
    let series = utils::csv_reader::read_bitcoin_address_series();
    println!("The current directory is {}", series);
}
