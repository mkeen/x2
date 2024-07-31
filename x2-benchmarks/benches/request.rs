use criterion::{criterion_group, criterion_main, Criterion};
use x2::prelude::*;
use x2::{model::users::Field, requests::csv};

pub fn single_set_csv(c: &mut Criterion) {
    c.bench_function("main bench", |b| {
        b.iter(|| {
            csv::<{ Field::COUNT }, Field>(&[
                Field::Description,
                Field::Location,
                Field::ProfileImageUrl,
                Field::MostRecentTweetId,
                Field::Description,
                Field::Location,
                Field::ProfileImageUrl,
                Field::MostRecentTweetId,
            ]);
        })
    });
}

pub fn multi_set_csv(c: &mut Criterion) {
    c.bench_function("main bench", |b| {
        b.iter(|| {
            csv::<{ Field::COUNT }, Field>(&[
                Field::Description,
                Field::Location,
                Field::ProfileImageUrl,
                Field::MostRecentTweetId,
                Field::Description,
                Field::Location,
                Field::ProfileImageUrl,
                Field::MostRecentTweetId,
            ]);

            csv::<{ Field::COUNT }, Field>(&[
                Field::Description,
                Field::Location,
                Field::ProfileImageUrl,
                Field::MostRecentTweetId,
                Field::Description,
                Field::Location,
                Field::ProfileImageUrl,
                Field::MostRecentTweetId,
            ]);

            csv::<{ Field::COUNT }, Field>(&[
                Field::Description,
                Field::Location,
                Field::ProfileImageUrl,
                Field::MostRecentTweetId,
                Field::Description,
                Field::Location,
                Field::ProfileImageUrl,
                Field::MostRecentTweetId,
            ]);

            csv::<{ Field::COUNT }, Field>(&[
                Field::Description,
                Field::Location,
                Field::ProfileImageUrl,
                Field::MostRecentTweetId,
                Field::Description,
                Field::Location,
                Field::ProfileImageUrl,
                Field::MostRecentTweetId,
            ]);

            csv::<{ Field::COUNT }, Field>(&[
                Field::Description,
                Field::Location,
                Field::ProfileImageUrl,
                Field::MostRecentTweetId,
                Field::Description,
                Field::Location,
                Field::ProfileImageUrl,
                Field::MostRecentTweetId,
            ]);
        })
    });
}

criterion_group!(benches, single_set_csv, multi_set_csv);
criterion_main!(benches);
