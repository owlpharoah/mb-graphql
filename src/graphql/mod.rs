use async_graphql::{EmptyMutation, EmptySubscription, Schema, dataloader::DataLoader};

use crate::graphql::loaders::entity::label::LabelLoader;
use crate::graphql::loaders::entity::medium::MediumLoader;
use crate::graphql::loaders::entity::release::ReleaseLoader;
use crate::graphql::loaders::entity::release_group::ReleaseGroupLoader;
use crate::graphql::loaders::entity::tracks::TrackLoader;
use crate::graphql::loaders::label_infos_by_release::LabelInfosByReleaseLoader;
use crate::graphql::loaders::relationship::medium_id_by_release::MediumIdByReleaseLoader;
use crate::graphql::loaders::relationship::release_group_id_by_artist::ReleaseGroupIdsByArtistLoader;
use crate::graphql::loaders::relationship::release_id_by_artist::ReleaseIdsByArtistLoader;
use crate::graphql::loaders::relationship::release_id_by_release_group::ReleaseIdByReleaseGroupLoader;
use crate::graphql::loaders::relationship::track_id_by_medium::TrackIdByMediumLoader;
use crate::graphql::query::QueryRoot;

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub mod loaders;
pub mod query;
pub mod types;

pub fn build_schema(pool: sqlx::PgPool) -> AppSchema {
    let rg_entity_loader = DataLoader::new(ReleaseGroupLoader { pool: pool.clone() }, tokio::spawn);
    let r_entity_loader = DataLoader::new(ReleaseLoader { pool: pool.clone() }, tokio::spawn);
    let l_entity_loader = DataLoader::new(LabelLoader { pool: pool.clone() }, tokio::spawn);
    let medium_entity_loader = DataLoader::new(MediumLoader { pool: pool.clone() }, tokio::spawn);
    let track_entity_loader = DataLoader::new(TrackLoader { pool: pool.clone() }, tokio::spawn);

    let rg_a_loader = DataLoader::new(
        ReleaseGroupIdsByArtistLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let r_a_loader = DataLoader::new(
        ReleaseIdsByArtistLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let r_rg_loader = DataLoader::new(
        ReleaseIdByReleaseGroupLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let li_r_loader = DataLoader::new(
        LabelInfosByReleaseLoader { pool: pool.clone() },
        tokio::spawn,
    );
    let medium_release_loader =
        DataLoader::new(MediumIdByReleaseLoader { pool: pool.clone() }, tokio::spawn);
    let track_medium_loader =
        DataLoader::new(TrackIdByMediumLoader { pool: pool.clone() }, tokio::spawn);

    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .limit_depth(5)
        .limit_complexity(200)
        .data(pool)
        .data(rg_entity_loader)
        .data(r_entity_loader)
        .data(l_entity_loader)
        .data(medium_entity_loader)
        .data(track_entity_loader)
        .data(rg_a_loader)
        .data(r_a_loader)
        .data(r_rg_loader)
        .data(li_r_loader)
        .data(medium_release_loader)
        .data(track_medium_loader)
        .finish()
}

pub fn build_schema_export() -> AppSchema {
    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .limit_depth(5)
        .limit_complexity(200)
        .finish()
}
