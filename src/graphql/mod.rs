use async_graphql::{EmptyMutation, EmptySubscription, Schema, dataloader::DataLoader};

use crate::graphql::loaders::release_by_release_group::ReleaseByGroupLoader;
use crate::graphql::loaders::release_group_by_artist::ReleaseGroupByArtistLoader;
use crate::graphql::query::QueryRoot;

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub mod loaders;
pub mod query;
pub mod types;

pub fn build_schema(pool: sqlx::PgPool) -> AppSchema {
    let rg_a_loader = DataLoader::new(
        ReleaseGroupByArtistLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let r_rg_loader = DataLoader::new(ReleaseByGroupLoader { pool: pool.clone() }, tokio::spawn);

    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .limit_depth(5)
        .limit_complexity(200)
        .data(pool)
        .data(rg_a_loader)
        .data(r_rg_loader)
        .finish()
}

pub fn build_schema_export() -> AppSchema {
    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .limit_depth(5)
        .limit_complexity(200)
        .finish()
}
