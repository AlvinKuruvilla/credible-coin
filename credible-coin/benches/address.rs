use credible_coin::utils::bitcoin_utils::generate_n_address_value_pairs;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn bench_address_generation(c: &mut Criterion) {
    c.bench_function("bench_address_generator", |b: &mut criterion::Bencher| {
        b.iter(|| {
            generate_n_address_value_pairs(1000000);
        })
    });
}
criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(10);
    targets = bench_address_generation
}
criterion_main!(benches);
