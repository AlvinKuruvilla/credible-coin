use credible_coin::utils;
fn main() {
    let series = utils::csv_utils::get_dataset_column_by_name("addresses");
    println!("The current directory is {}", series);
}
