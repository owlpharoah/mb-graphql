use criterion::{Criterion, criterion_group, criterion_main};
use serde_json::json;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;
use tokio::runtime::Runtime;

use async_graphql::{Request, Variables};
use mb_graphql::graphql::{AppSchema, build_schema};

#[path = "../tests/queries/mod.rs"]
mod queries;

use queries::{artist, mbids};

pub const ARTIST_FULL_DISCOGRAPHY: &str = r#"
    query ArtistFullDiscography($mbid: [String!]!) {
        artist(mbid: $mbid) {
            name
            sortName
            releaseGroups(first: 10) {
                name
                type
                firstReleaseDate { year month day }
                releases(first: 3) {
                    name
                    status
                    date { year }
                    medium(first: 2) {
                        name
                        trackCount
                        tracks(first: 2) {
                            name
                            position
                            length
                        }
                    }
                }
            }
        }
    }
"#;

pub const MULTI_ENTITY_CROSS_JOIN: &str = r#"
    query MultiEntityCrossJoin($mbid1: [String!]!, $mbid2: [String!]!) {
        a0: artist(mbid: $mbid1) { ...ArtistFull }
        a1: artist(mbid: $mbid2) { ...ArtistFull }
    }
    fragment ArtistFull on Artist {
        mbid
        name
        sortName
        tags { name count }
        genres(first: 10) { name }
        rating { value votesCount }
        releaseGroups(first: 3) {
            name
            type
            releases(first: 5) {
                name
                date { year }
                artistCredit { name joinPhrase }
            }
        }
    }
"#;

async fn test_pool() -> PgPool {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
        "postgres://musicbrainz:musicbrainz@localhost:5432/musicbrainz_db".to_string()
    });

    PgPoolOptions::new()
        .max_connections(20)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(600))
        .connect(&db_url)
        .await
        .expect("Failed to connect to test database")
}

async fn run_query(schema: &AppSchema, query: &str, vars: serde_json::Value) {
    let variables = Variables::from_json(vars);
    let request = Request::new(query).variables(variables);
    let response = schema.execute(request).await;
    assert!(
        response.errors.is_empty(),
        "Query failed: {:?}",
        response.errors
    );
}

fn bench_queries(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let pool = rt.block_on(test_pool());
    let schema = build_schema(pool);

    let mut group = c.benchmark_group("graphql_queries");
    group.sample_size(10);

    // basic lookup
    group.bench_function("artist_basic", |b| {
        b.to_async(&rt).iter(|| {
            run_query(
                &schema,
                artist::ARTIST_BASIC,
                json!({ "mbid": [mbids::YE] }),
            )
        })
    });

    // batch lookup
    group.bench_function("artist_batch", |b| {
        b.to_async(&rt).iter(|| {
            run_query(
                &schema,
                artist::ARTIST_BATCH,
                json!({ "mbid": [mbids::YE, mbids::ARCTIC_MONKEYS] }),
            )
        })
    });

    // meduim complexity
    group.bench_function("artist_release_groups", |b| {
        b.to_async(&rt).iter(|| {
            run_query(
                &schema,
                artist::ARTIST_RELEASE_GROUPS_AND_RELEASES,
                json!({ "mbid": [mbids::YE] }),
            )
        })
    });

    // high load queries
    group.bench_function("load_artist_full_discography", |b| {
        b.to_async(&rt).iter(|| {
            run_query(
                &schema,
                ARTIST_FULL_DISCOGRAPHY,
                json!({ "mbid": [mbids::ARCTIC_MONKEYS] }),
            )
        })
    });

    group.bench_function("load_multi_entity_cross_join", |b| {
        b.to_async(&rt).iter(|| {
            run_query(
                &schema,
                MULTI_ENTITY_CROSS_JOIN,
                json!({ "mbid1": [mbids::YE], "mbid2": [mbids::ARCTIC_MONKEYS] }),
            )
        })
    });

    group.finish();
}

criterion_group!(benches, bench_queries);
criterion_main!(benches);
