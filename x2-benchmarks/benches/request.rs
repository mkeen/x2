use criterion::{criterion_group, criterion_main, Criterion};
use x2::prelude::*;
use x2::{model::users::Field, requests::collect_csv};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("main bench", |b| {
        b.iter(|| {
            collect_csv::<Field, { Field::COUNT }>(&[
                Field::Description,
                Field::Location,
                Field::ProfileImageUrl,
                Field::MostRecentTweetId,
            ]);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
