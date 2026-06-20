use async_graphql::{ComplexObject, Context, Object, SimpleObject, dataloader::DataLoader};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::graphql::loaders::entity::medium::MediumLoader;
use crate::graphql::loaders::label_infos_by_release::LabelInfosByReleaseLoader;
use crate::graphql::loaders::relationship::medium_id_by_release::MediumIdByReleaseLoader;
use crate::graphql::loaders::release_event_by_release::ReleaseEventsByReleaseLoader;
use crate::graphql::types::common::{Medium, ReleaseEvent};
use crate::graphql::types::{
    self,
    common::LabelInfo,
    release_group::{ReleaseGroup, ReleaseGroupRow},
};
use types::common::PartialDate;

#[derive(sqlx::FromRow)]
pub struct ReleaseRow {
    pub id: i32,
    pub gid: Uuid,
    pub name: String,
    pub artist_credit: i32,
    pub release_group: i32,
    pub status: Option<i32>,
    pub packaging: Option<i32>,
    pub quality: i16, //default is -1
    pub language: Option<i32>,
    pub script: Option<i32>,
    pub barcode: Option<String>,
    pub comment: Option<String>,
}
#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Release {
    pub mbid: Uuid,
    pub name: String,
    pub status: Option<i32>,
    pub packaging: Option<i32>,
    pub quality: i16, //default is -1
    pub language: Option<i32>,
    pub script: Option<i32>,
    pub barcode: Option<String>,
    pub disambiguation: Option<String>,

    #[graphql(skip)]
    pub id: i32,
    #[graphql(skip)]
    pub artist_credit: i32,
    #[graphql(skip)]
    pub release_group: i32,
}

impl From<ReleaseRow> for Release {
    fn from(r: ReleaseRow) -> Self {
        Self {
            mbid: r.gid,
            name: r.name,
            disambiguation: r.comment,
            status: r.status,
            packaging: r.packaging,
            quality: r.quality,
            barcode: r.barcode,
            language: r.language,
            script: r.script,
            release_group: r.release_group,
            id: r.id,
            artist_credit: r.artist_credit,
        }
    }
}

#[derive(Default)]
pub struct ReleaseQuery;

#[Object]
impl ReleaseQuery {
    async fn release(
        &self,
        ctx: &Context<'_>,
        mbid: String,
    ) -> async_graphql::Result<Option<Release>> {
        let pool = ctx.data::<PgPool>()?;
        let uuid = Uuid::parse_str(&mbid)?;

        let row = sqlx::query_as::<_, ReleaseRow>(
            "SELECT
                id,
                gid,
                name,
                artist_credit,
                release_group,
                status,
                packaging,
                quality,
                language,
                script,
                barcode,
                comment
            FROM release
            WHERE gid = $1",
        )
        .bind(uuid)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(Release::from))
    }

    async fn releases(
        &self,
        ctx: &Context<'_>,
        mbids: Vec<String>,
    ) -> async_graphql::Result<Vec<Release>> {
        let pool = ctx.data::<PgPool>()?;
        let uuids: Vec<Uuid> = mbids
            .iter()
            .map(|s| Uuid::parse_str(s))
            .collect::<Result<_, _>>()?;

        let rows = sqlx::query_as::<_, ReleaseRow>(
            "SELECT
                id,
                gid,
                name,
                artist_credit,
                release_group,
                status,
                packaging,
                quality,
                language,
                script,
                barcode,
                comment
            FROM release
            WHERE gid = ANY($1)",
        )
        .bind(&uuids)
        .fetch_all(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(rows.into_iter().map(Release::from).collect())
    }
}

#[ComplexObject]
impl Release {
    async fn date(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<PartialDate>> {
        let pool = ctx.data::<PgPool>()?;

        let row: Option<(Option<i16>, Option<i16>, Option<i16>)> = sqlx::query_as(
            "SELECT year,month,day
            FROM release_first_release_date
            WHERE release = $1",
        )
        .bind(self.id)
        .fetch_optional(pool)
        .await?;

        let pdate = match row {
            Some((year, month, day)) => PartialDate::from_parts(year, month, day),
            None => None,
        };

        Ok(pdate)
    }

    async fn asin(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Vec<String>>> {
        let pool = ctx.data::<PgPool>()?;

        let row: Vec<String> = sqlx::query_scalar(
            "SELECT u.url
            FROM l_release_url lru
            JOIN link l ON l.id = lru.link
            JOIN link_type lt ON lt.id = l.link_type
            JOIN url u ON u.id = lru.entity1
            WHERE lru.entity0 = $1
              AND lt.id = 77;",
        )
        .bind(self.id)
        .fetch_all(pool)
        .await?;

        Ok(if row.is_empty() { None } else { Some(row) })
    }

    async fn country(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<String>> {
        let pool = ctx.data::<PgPool>()?;

        let row: Option<String> = sqlx::query_scalar(
            "SELECT
                a.name
            FROM release_country rc
            JOIN area a
              ON a.id = rc.country
            WHERE rc.release = $1;",
        )
        .bind(self.id)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    //backward
    async fn release_group(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Option<ReleaseGroup>> {
        let pool = ctx.data::<PgPool>()?;

        let row = sqlx::query_as::<_, ReleaseGroupRow>(
            "SELECT rg.id, rg.gid, rg.name, rg.comment, rg.type, rg.artist_credit
                        FROM release_group rg JOIN release r ON rg.id = r.release_group
                        WHERE r.id = $1",
        )
        .bind(self.id)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(ReleaseGroup::from))
    }

    async fn label_info(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<LabelInfo>> {
        let loader = ctx.data::<DataLoader<LabelInfosByReleaseLoader>>()?;

        Ok(loader.load_one(self.id).await?.unwrap_or_default())
    }

    async fn medium(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Medium>> {
        let medium_ids_loader = ctx.data::<DataLoader<MediumIdByReleaseLoader>>()?;
        let medium_ids = medium_ids_loader
            .load_one(self.id)
            .await?
            .unwrap_or_default();

        if medium_ids.is_empty() {
            return Ok(vec![]);
        }
        let medium_loader = ctx.data::<DataLoader<MediumLoader>>()?;
        let mediums_map = medium_loader.load_many(medium_ids.clone()).await?;

        Ok(medium_ids
            .into_iter()
            .filter_map(|id| mediums_map.get(&id).cloned())
            .collect())
    }

    async fn release_events(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<ReleaseEvent>> {
        let loader = ctx.data::<DataLoader<ReleaseEventsByReleaseLoader>>()?;

        Ok(loader.load_one(self.id).await?.unwrap_or_default())
    }
}
