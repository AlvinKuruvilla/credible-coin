use std::{fs, path::Path};

use credible_coin::cli::exchange::asset_database::create_exchange_database;
use criterion::{criterion_group, criterion_main, Criterion};
pub fn bench_repeated_exchange_db_create(c: &mut Criterion) {
    c.bench_function("repeat_exchange_db_create", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                create_exchange_database(
                    "BigQuery Bitcoin Historical Data - outputs.csv",
                    "test.csv",
                    20,
                );
                Path::new("test.csv")
                    .try_exists()
                    .expect("Can't find the file");
                fs::remove_file("test.csv").expect("Could not delete file");
            }
        })
    });
}

criterion_group!(benches, bench_repeated_exchange_db_create);
criterion_main!(benches);
