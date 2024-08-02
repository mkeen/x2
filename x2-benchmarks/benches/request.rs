use criterion::{criterion_group, criterion_main, Criterion};
use x2::{model::users::Field, requests::csv};

pub fn single_set_csv_big(c: &mut Criterion) {
    c.bench_function("main bench", |b| {
        b.iter(|| {
            csv(&[
                Field::CreatedAt,
                Field::Description,
                Field::Entities,
                Field::Id,
                Field::Location,
                Field::MostRecentTweetId,
                Field::Name,
                Field::PinnedTweetId,
                Field::ProfileImageUrl,
                Field::Protected,
                Field::PublicMetrics,
                Field::Url,
                Field::Username,
                Field::Verified,
                Field::VerifiedType,
                Field::Withheld,
            ]);
        })
    });
}

pub fn multi_set_csv_allbig(c: &mut Criterion) {
    c.bench_function("main bench", |b| {
        b.iter(|| {
            csv(&[
                Field::CreatedAt,
                Field::Description,
                Field::Entities,
                Field::Id,
                Field::Location,
                Field::MostRecentTweetId,
                Field::Name,
                Field::PinnedTweetId,
                Field::ProfileImageUrl,
                Field::Protected,
                Field::PublicMetrics,
                Field::Url,
                Field::Username,
                Field::Verified,
                Field::VerifiedType,
                Field::Withheld,
            ]);

            csv(&[
                Field::CreatedAt,
                Field::Description,
                Field::Entities,
                Field::Id,
                Field::Location,
                Field::MostRecentTweetId,
                Field::Name,
                Field::PinnedTweetId,
                Field::ProfileImageUrl,
                Field::Protected,
                Field::PublicMetrics,
                Field::Url,
                Field::Username,
                Field::Verified,
                Field::VerifiedType,
                Field::Withheld,
            ]);

            csv(&[
                Field::CreatedAt,
                Field::Description,
                Field::Entities,
                Field::Id,
                Field::Location,
                Field::MostRecentTweetId,
                Field::Name,
                Field::PinnedTweetId,
                Field::ProfileImageUrl,
                Field::Protected,
                Field::PublicMetrics,
                Field::Url,
                Field::Username,
                Field::Verified,
                Field::VerifiedType,
                Field::Withheld,
            ]);

            csv(&[
                Field::CreatedAt,
                Field::Description,
                Field::Entities,
                Field::Id,
                Field::Location,
                Field::MostRecentTweetId,
                Field::Name,
                Field::PinnedTweetId,
                Field::ProfileImageUrl,
                Field::Protected,
                Field::PublicMetrics,
                Field::Url,
                Field::Username,
                Field::Verified,
                Field::VerifiedType,
                Field::Withheld,
            ]);

            csv(&[
                Field::CreatedAt,
                Field::Description,
                Field::Entities,
                Field::Id,
                Field::Location,
                Field::MostRecentTweetId,
                Field::Name,
                Field::PinnedTweetId,
                Field::ProfileImageUrl,
                Field::Protected,
                Field::PublicMetrics,
                Field::Url,
                Field::Username,
                Field::Verified,
                Field::VerifiedType,
                Field::Withheld,
            ]);
        })
    });
}

pub fn multi_set_csv_varies(c: &mut Criterion) {
    c.bench_function("main bench", |b| {
        b.iter(|| {
            csv(&[Field::CreatedAt]);

            csv(&[
                Field::ProfileImageUrl,
                Field::Protected,
                Field::PublicMetrics,
                Field::Url,
                Field::Username,
            ]);

            csv(&[
                Field::CreatedAt,
                Field::Description,
                Field::Entities,
                Field::Id,
                Field::Location,
                Field::MostRecentTweetId,
                Field::Name,
                Field::PinnedTweetId,
                Field::ProfileImageUrl,
                Field::Protected,
                Field::PublicMetrics,
                Field::Url,
                Field::Username,
                Field::Verified,
                Field::VerifiedType,
                Field::Withheld,
            ]);

            csv(&[Field::CreatedAt, Field::Url, Field::Username]);

            csv(&[
                Field::CreatedAt,
                Field::Description,
                Field::Entities,
                Field::Id,
                Field::Name,
                Field::PinnedTweetId,
                Field::ProfileImageUrl,
                Field::Protected,
                Field::PublicMetrics,
            ]);
        })
    });
}

criterion_group!(
    benches,
    single_set_csv_big,
    multi_set_csv_varies,
    multi_set_csv_allbig
);
criterion_main!(benches);
