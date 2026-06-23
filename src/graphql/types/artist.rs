use async_graphql::{ComplexObject, Context, Object, SimpleObject, dataloader::DataLoader};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;

use crate::graphql::{
    loaders::{
        entity::{release::ReleaseLoader, release_group::ReleaseGroupLoader, tag::TagLoader},
        relationship::{
            release_group_id_by_artist::ReleaseGroupIdsByArtistLoader,
            release_id_by_artist::ReleaseIdsByArtistLoader, tag_id_by_artist::TagIdsByArtistLoader,
        },
    },
    types::{self, common::Tag, release::Release, release_group::ReleaseGroup},
};
use types::common::PartialDate;

#[derive(sqlx::FromRow)]
struct ArtistRow {
    id: i32,
    gid: Uuid,
    name: String,
    sort_name: String,
    comment: Option<String>,
    #[sqlx(rename = "type")]
    artist_type: Option<i32>,
    gender: Option<i32>,
    area: Option<i32>,
    ended: bool,
    begin_date_year: Option<i16>,
    begin_date_month: Option<i16>,
    begin_date_day: Option<i16>,
    end_date_year: Option<i16>,
    end_date_month: Option<i16>,
    end_date_day: Option<i16>,

    begin_area: Option<i32>,
    end_area: Option<i32>,
}

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Artist {
    pub mbid: Uuid,
    pub name: String,
    #[graphql(name = "sortName")]
    pub sort_name: String,
    pub disambiguation: Option<String>,
    #[graphql(name = "type")]
    pub artist_type: Option<i32>,
    pub gender: Option<i32>,
    pub ended: bool,
    #[graphql(name = "beginDate")]
    pub begin_date: Option<PartialDate>,
    #[graphql(name = "endDate")]
    pub end_date: Option<PartialDate>,

    #[graphql(skip)]
    pub id: i32,
    #[graphql(skip)]
    pub area: Option<i32>,
    #[graphql(skip)]
    pub begin_area: Option<i32>,
    #[graphql(skip)]
    pub end_area: Option<i32>,
}

impl From<ArtistRow> for Artist {
    fn from(r: ArtistRow) -> Self {
        Self {
            mbid: r.gid,
            name: r.name,
            sort_name: r.sort_name,
            disambiguation: r.comment,
            artist_type: r.artist_type,
            gender: r.gender,
            area: r.area,
            ended: r.ended,
            begin_date: PartialDate::from_parts(
                r.begin_date_year,
                r.begin_date_month,
                r.begin_date_day,
            ),
            end_date: PartialDate::from_parts(r.end_date_year, r.end_date_month, r.end_date_day),
            id: r.id,
            begin_area: r.begin_area,
            end_area: r.end_area,
        }
    }
}

#[derive(Default)]
pub struct ArtistQuery;

#[Object]
impl ArtistQuery {
    async fn artist(
        &self,
        ctx: &Context<'_>,
        mbid: String,
    ) -> async_graphql::Result<Option<Artist>> {
        let pool = ctx.data::<PgPool>()?;
        let uuid = Uuid::parse_str(&mbid)?;

        let row = sqlx::query_as::<_, ArtistRow>(
            "SELECT id, gid, name, sort_name, comment,type, gender, area, ended, begin_date_year, begin_date_month, begin_date_day, end_date_year, end_date_month, end_date_day,begin_area,end_area
                        FROM artist
                        WHERE gid = $1",
        )
        .bind(uuid)
        .fetch_optional(pool)
        .await.map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(row.map(Artist::from))
    }

    async fn artists(
        &self,
        ctx: &Context<'_>,
        mbids: Vec<String>,
    ) -> async_graphql::Result<Vec<Artist>> {
        let pool = ctx.data::<PgPool>()?;
        let uuids: Vec<Uuid> = mbids
            .iter()
            .map(|s| Uuid::parse_str(s))
            .collect::<Result<_, _>>()?;

        let rows = sqlx::query_as::<_, ArtistRow>(
            "SELECT id, gid, name, sort_name, comment,type, gender, area, ended, begin_date_year, begin_date_month, begin_date_day, end_date_year, end_date_month, end_date_day,begin_area,end_area
                        FROM artist
                        WHERE gid = ANY($1)",
        )
        .bind(&uuids)
        .fetch_all(pool)
        .await.map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(rows.into_iter().map(Artist::from).collect())
    }
}

#[ComplexObject]
impl Artist {
    async fn release_groups(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<ReleaseGroup>> {
        info!(artist_id = self.id, "Artist.release_groups resolver called");

        let artist_ids = ctx.data::<DataLoader<ReleaseGroupIdsByArtistLoader>>()?;
        let ids = artist_ids.load_one(self.id).await?.unwrap_or_default();

        info!(
            artist_id = self.id,
            release_group_count = ids.len(),
            "Release group ids loaded"
        );

        if ids.is_empty() {
            return Ok(vec![]);
        }

        let rg_loader = ctx.data::<DataLoader<ReleaseGroupLoader>>()?;
        let rg_map = rg_loader.load_many(ids.clone()).await?;

        Ok(ids
            .into_iter()
            .filter_map(|id| rg_map.get(&id).cloned())
            .collect())
    }
    async fn releases(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Release>> {
        info!(artist_id = self.id, "Artist.releases resolver called");
        let artist_ids = ctx.data::<DataLoader<ReleaseIdsByArtistLoader>>()?;
        let ids = artist_ids.load_one(self.id).await?.unwrap_or_default();
        info!(
            artist_id = self.id,
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
    async fn tags(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Tag>> {
        info!(artist_id = self.id, "Artist.tags resolver called");

        let id_loader = ctx.data::<DataLoader<TagIdsByArtistLoader>>()?;
        let refs = id_loader.load_one(self.id).await?.unwrap_or_default();

        if refs.is_empty() {
            return Ok(vec![]);
        }

        let tag_ids: Vec<i32> = refs.iter().map(|r| r.tag_id).collect();
        let name_loader = ctx.data::<DataLoader<TagLoader>>()?;
        let name_map = name_loader.load_many(tag_ids).await?;

        Ok(refs
            .into_iter()
            .filter_map(|r| {
                name_map.get(&r.tag_id).map(|name| Tag {
                    name: name.clone(),
                    count: r.count,
                })
            })
            .collect())
    }
}
