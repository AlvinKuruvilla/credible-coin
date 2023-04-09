use credible_coin::utils;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn bench_read_bitcoin_address_dataframe(c: &mut Criterion) {
    c.bench_function("read_bitcoin_address_dataframe", |b| {
        b.iter(|| {
            utils::csv_utils::read_bitcoin_address_dataframe(
                "BigQuery Bitcoin Historical Data - outputs.csv",
            )
        })
    });
}
pub fn bench_addresses_and_values_as_vectors(c: &mut Criterion) {
    c.bench_function("addresses_and_values_as_vectors", |b| {
        b.iter(|| {
            utils::csv_utils::addresses_and_values_as_vectors(
                "BigQuery Bitcoin Historical Data - outputs.csv",
            )
        })
    });
}
criterion_group!(
    benches,
    bench_read_bitcoin_address_dataframe,
    bench_addresses_and_values_as_vectors
);
criterion_main!(benches);
