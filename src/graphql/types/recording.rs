use crate::graphql::{
    loaders::{
        entity::release::ReleaseLoader,
        relationship::release_id_by_recording::ReleaseIdsByRecordingLoader,
    },
    types::{self, release::Release},
};
use async_graphql::{ComplexObject, Context, Object, SimpleObject, dataloader::DataLoader};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::info;
use types::common::PartialDate;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct RecordingRow {
    pub id: i32,
    pub gid: Uuid,
    pub name: String,
    pub artist_credit: i32,
    pub comment: Option<String>,
    pub length: Option<i32>,
    pub video: bool,
}
#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Recording {
    pub mbid: Uuid,
    pub name: String,
    pub disambiguation: Option<String>,
    pub length: Option<i32>,
    pub video: bool,

    #[graphql(skip)]
    pub id: i32,
    #[graphql(skip)]
    pub artist_credit: i32,
}

impl From<RecordingRow> for Recording {
    fn from(r: RecordingRow) -> Self {
        Self {
            mbid: r.gid,
            name: r.name,
            disambiguation: r.comment,
            length: r.length,
            video: r.video,
            id: r.id,
            artist_credit: r.artist_credit,
        }
    }
}

#[derive(Default)]
pub struct RecordingQuery;

#[Object]
impl RecordingQuery {
    async fn recording(
        &self,
        ctx: &Context<'_>,
        mbid: String,
    ) -> async_graphql::Result<Option<Recording>> {
        let pool = ctx.data::<PgPool>()?;
        let uuid = Uuid::parse_str(&mbid)?;

        let row = sqlx::query_as::<_, RecordingRow>(
            "SELECT
                id,
                gid,
                name,
                artist_credit,
                comment,
                length,
                video
            FROM recording
            WHERE gid = $1",
        )
        .bind(uuid)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(Recording::from))
    }

    async fn recordings(
        &self,
        ctx: &Context<'_>,
        mbids: Vec<String>,
    ) -> async_graphql::Result<Vec<Recording>> {
        let pool = ctx.data::<PgPool>()?;
        let uuids: Vec<Uuid> = mbids
            .iter()
            .map(|s| Uuid::parse_str(s))
            .collect::<Result<_, _>>()?;

        let rows = sqlx::query_as::<_, RecordingRow>(
            "SELECT
                id,
                gid,
                name,
                artist_credit,
                comment,
                length,
                video
            FROM recording
            WHERE gid = ANY($1)",
        )
        .bind(&uuids)
        .fetch_all(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(rows.into_iter().map(Recording::from).collect())
    }
}

#[ComplexObject]
impl Recording {
    async fn first_release_date(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Option<PartialDate>> {
        let pool = ctx.data::<PgPool>()?;

        let row: Option<(Option<i16>, Option<i16>, Option<i16>)> = sqlx::query_as(
            "SELECT year,month,day
            FROM recording_first_release_date
            WHERE recording = $1",
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
    async fn isrc(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<String>> {
        let pool = ctx.data::<PgPool>()?;

        let rows = sqlx::query_scalar(
            "
            SELECT isrc
            FROM isrc
            WHERE recording = $1
            ",
        )
        .bind(self.id)
        .fetch_optional(pool)
        .await?;

        Ok(rows)
    }

    async fn release(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Release>> {
        info!(recording_id = self.id, "Recording.releases resolver called");

        let release_id_loader = ctx.data::<DataLoader<ReleaseIdsByRecordingLoader>>()?;
        let release_id = release_id_loader
            .load_one(self.id)
            .await?
            .unwrap_or_default();
        info!(
            recording_id = self.id,
            releases_count = release_id.len(),
            "Release ids loaded"
        );
        if release_id.is_empty() {
            return Ok(vec![]);
        }

        let release_loader = ctx.data::<DataLoader<ReleaseLoader>>()?;
        let release_map = release_loader.load_many(release_id.clone()).await?;

        Ok(release_id
            .into_iter()
            .filter_map(|id| release_map.get(&id).cloned())
            .collect())
    }
}
