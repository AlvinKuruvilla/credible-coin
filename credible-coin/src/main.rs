use credible_coin::utils;
fn main() {
    // let series = utils::csv_utils::get_dataset_column_by_name("addresses");
    // println!("The series is {}", series);

    let (v1, v2) = utils::csv_utils::addresses_and_values_as_vectors(
        "BigQuery Bitcoin Historical Data - outputs.csv",
    );
    println!("V1 {:?}", v2);
}
