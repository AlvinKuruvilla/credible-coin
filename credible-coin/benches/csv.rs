use credible_coin::utils;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn bench_addresses_and_values_as_vectors(c: &mut Criterion) {
    c.bench_function("addresses_and_values_as_vectors", |b| {
        b.iter(|| {
            utils::csv_utils::addresses_and_values_as_vectors(
                "../scripts/generated/exchange_secret.csv",
            )
        })
    });
}
pub fn bench_make_value_vector(c: &mut Criterion) {
    c.bench_function("bench_make_value_vector", |b| {
        b.iter(|| utils::csv_utils::make_value_vector("../scripts/generated/exchange_secret.csv"))
    });
}
pub fn bench_make_address_vector(c: &mut Criterion) {
    c.bench_function("bench_make_address_vector", |b| {
        b.iter(|| utils::csv_utils::make_address_vector("../scripts/generated/exchange_secret.csv"))
    });
}
criterion_group!(
    benches,
    bench_addresses_and_values_as_vectors,
    bench_make_address_vector,
    bench_make_value_vector
);
criterion_main!(benches);
