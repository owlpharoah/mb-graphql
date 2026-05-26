use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use crate::graphql::query::QueryRoot;

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub mod query;
pub mod types;

pub fn build_schema(pool: sqlx::PgPool) -> AppSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .limit_depth(5)
        .limit_complexity(200)
        .data(pool)
        .finish()
}

pub fn build_schema_export() -> AppSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .limit_depth(5)
        .limit_complexity(200)
        .finish()
}
