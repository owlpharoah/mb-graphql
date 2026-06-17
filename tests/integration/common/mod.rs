use async_graphql::{EmptyMutation, EmptySubscription, Schema, dataloader::DataLoader};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

use mb_graphql::graphql::loaders::release_group_by_artist::ReleaseGroupByArtistLoader;
use mb_graphql::graphql::query::QueryRoot;

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema(pool: sqlx::PgPool) -> AppSchema {
    let loader = DataLoader::new(
        ReleaseGroupByArtistLoader { pool: pool.clone() },
        tokio::spawn,
    );

    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .limit_depth(5)
        .limit_complexity(200)
        .data(pool)
        .data(loader)
        .finish()
}

pub async fn test_pool() -> Result<PgPool, sqlx::Error> {
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
}

pub async fn test_schema() -> AppSchema {
    let pool = test_pool()
        .await
        .expect("failed to connect to test database");

    build_schema(pool)
}
