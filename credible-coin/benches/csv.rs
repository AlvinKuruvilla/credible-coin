use credible_coin::utils;
use criterion::{criterion_group, criterion_main, Criterion};

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
    bench_addresses_and_values_as_vectors
);
criterion_main!(benches);
