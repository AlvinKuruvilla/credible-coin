use credible_coin::utils::db_funcs::{load_db, load_merkle_leaves};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn bench_build_tree(c: &mut Criterion) {
    c.bench_function("bench_build_tree", |b| {
        b.iter(|| {
            let merkle_leaves =
                load_merkle_leaves("BigQuery Bitcoin Historical Data - outputs.csv");
            load_db(merkle_leaves.clone());
        })
    });
}
pub fn bench_load_leaves(c: &mut Criterion) {
    c.bench_function("bench_load_leaves", |b| {
        b.iter(|| {
            load_merkle_leaves("BigQuery Bitcoin Historical Data - outputs.csv");
        })
    });
}
criterion_group!(benches, bench_build_tree, bench_load_leaves);
criterion_main!(benches);