use async_graphql::{ComplexObject, Context, Object, SimpleObject, dataloader::DataLoader};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;

use crate::graphql::{
    loaders::{
        entity::release::ReleaseLoader, rating_release_group::ReleaseGroupRatingLoader,
        relationship::release_id_by_release_group::ReleaseIdByReleaseGroupLoader,
    },
    types::{self, common::Rating, release::Release},
};
use types::common::PartialDate;

#[derive(sqlx::FromRow)]
pub struct ReleaseGroupRow {
    id: i32,
    gid: Uuid,
    name: String,
    artist_credit: i32,
    comment: Option<String>,
    #[sqlx(rename = "type")]
    release_group_type: Option<i32>,
}
#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct ReleaseGroup {
    pub mbid: Uuid,
    pub name: String,
    pub disambiguation: Option<String>,
    #[graphql(name = "type")]
    pub release_group_type: Option<i32>,

    #[graphql(skip)]
    pub id: i32,
    #[graphql(skip)]
    pub artist_credit: i32,
}

impl From<ReleaseGroupRow> for ReleaseGroup {
    fn from(r: ReleaseGroupRow) -> Self {
        Self {
            mbid: r.gid,
            name: r.name,
            disambiguation: r.comment,
            release_group_type: r.release_group_type,
            id: r.id,
            artist_credit: r.artist_credit,
        }
    }
}

#[derive(Default)]
pub struct ReleaseGroupQuery;

#[Object]
impl ReleaseGroupQuery {
    async fn release_group(
        &self,
        ctx: &Context<'_>,
        mbid: String,
    ) -> async_graphql::Result<Option<ReleaseGroup>> {
        let pool = ctx.data::<PgPool>()?;
        let uuid = Uuid::parse_str(&mbid)?;

        let row = sqlx::query_as::<_, ReleaseGroupRow>(
            "SELECT id, gid, name,comment,type,artist_credit
                        FROM release_group
                        WHERE gid = $1",
        )
        .bind(uuid)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(ReleaseGroup::from))
    }

    async fn release_groups(
        &self,
        ctx: &Context<'_>,
        mbids: Vec<String>,
    ) -> async_graphql::Result<Vec<ReleaseGroup>> {
        let pool = ctx.data::<PgPool>()?;
        let uuids: Vec<Uuid> = mbids
            .iter()
            .map(|s| Uuid::parse_str(s))
            .collect::<Result<_, _>>()?;

        let rows = sqlx::query_as::<_, ReleaseGroupRow>(
            "SELECT id, gid, name,comment,type,artist_credit
                        FROM release_group
                        WHERE gid = ANY($1)",
        )
        .bind(&uuids)
        .fetch_all(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(rows.into_iter().map(ReleaseGroup::from).collect())
    }
}

#[ComplexObject]
impl ReleaseGroup {
    async fn secondary_type(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Vec<i16>>> {
        let pool = ctx.data::<PgPool>()?;

        // this second option cause we need to handle three states:
        // row is none None
        // row is found and vec si present Some(vec)
        // row is found and vec is absent Some(None)
        let row: Option<Option<Vec<i16>>> = sqlx::query_scalar(
            "SELECT secondary_types
            FROM artist_release_group WHERE release_group = $1",
        )
        .bind(self.id)
        .fetch_optional(pool)
        .await?;

        Ok(row.flatten())
    }

    async fn first_release_date(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Option<PartialDate>> {
        let pool = ctx.data::<PgPool>()?;

        let row: Option<Option<i32>> = sqlx::query_scalar(
            "SELECT first_release_date
            FROM artist_release_group
            WHERE release_group = $1",
        )
        .bind(self.id)
        .fetch_optional(pool)
        .await?;

        let pdate = row.unwrap_or(None).and_then(|date| {
            PartialDate::from_parts(
                Some((date / 10000) as i16),
                Some(((date / 100) % 100) as i16),
                Some((date % 100) as i16),
            )
        });

        Ok(pdate)
    }

    async fn releases(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Release>> {
        info!(rg_id = self.id, "rg.releases resolver called");

        let rg_ids = ctx.data::<DataLoader<ReleaseIdByReleaseGroupLoader>>()?;
        let ids = rg_ids.load_one(self.id).await?.unwrap_or_default();
        info!(
            rg_id = self.id,
            releases_count = ids.len(),
            "Release ids loaded"
        );
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let r_loader = ctx.data::<DataLoader<ReleaseLoader>>()?;
        let r_map = r_loader.load_many(ids.clone()).await?;

        Ok(ids
            .into_iter()
            .filter_map(|id| r_map.get(&id).cloned())
            .collect())
    }
    async fn rating(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Rating>> {
        info!(rg_id = self.id, "ReleaseGroup.rating resolver called");
        let loader = ctx.data::<DataLoader<ReleaseGroupRatingLoader>>()?;
        Ok(loader.load_one(self.id).await?)
    }
}
