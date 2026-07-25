use async_graphql::{Request, ServerError, Variables};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

use mb_graphql::graphql::{AppSchema, build_schema};

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

pub async fn run(schema: &AppSchema, query: &str, vars: serde_json::Value) -> serde_json::Value {
    let variables = Variables::from_json(vars);
    let request = Request::new(query).variables(variables);
    let response = schema.execute(request).await;
    assert!(
        response.errors.is_empty(),
        "expected no errors, got: {:#?}",
        response.errors
    );
    response
        .data
        .into_json()
        .expect("response data should convert to JSON")
}

pub async fn run_expect_error(
    schema: &AppSchema,
    query: &str,
    vars: serde_json::Value,
) -> Vec<ServerError> {
    let variables = Variables::from_json(vars);
    let request = Request::new(query).variables(variables);
    let response = schema.execute(request).await;
    assert!(
        !response.errors.is_empty(),
        "expected the query to fail, but got data: {:#?}",
        response.data
    );
    response.errors
}

pub fn find_by_mbid<'a>(items: &'a [serde_json::Value], mbid: &str) -> &'a serde_json::Value {
    items
        .iter()
        .find(|item| item["mbid"] == mbid)
        .unwrap_or_else(|| panic!("expected an entity with mbid {mbid}, got: {items:#?}"))
}
