use crate::graphql::{
    loaders::{
        annotations_label::LabelAnnotationLoader,
        entity::{area::AreaLoader, genre::GenreLoader, release::ReleaseLoader},
        rating_label::LabelRatingLoader,
        relationship::{
            area_id_by_label::AreaIdByLabelLoader, genre_id_by_label::GenreIdsByLabelLoader,
            release_id_by_label::ReleaseIdsByLabelLoader,
        },
    },
    types::{
        self,
        area::Area,
        common::{Genre, Rating},
        release::Release,
    },
};
use async_graphql::{ComplexObject, Context, Object, SimpleObject, dataloader::DataLoader};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::info;
use types::common::PartialDate;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct LabelRow {
    pub id: i32,
    pub gid: Uuid,
    pub name: String,
    pub begin_date_year: Option<i16>,
    pub begin_date_month: Option<i16>,
    pub begin_date_day: Option<i16>,
    pub end_date_year: Option<i16>,
    pub end_date_month: Option<i16>,
    pub end_date_day: Option<i16>,
    #[sqlx(rename = "type")]
    pub label_type: Option<i32>,
    pub comment: Option<String>,
    pub ended: bool,
    pub label_code: Option<i32>,
}
#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Label {
    pub mbid: Uuid,
    pub name: String,
    pub disambiguation: Option<String>,
    #[graphql(name = "type")]
    pub label_type: Option<i32>,
    pub ended: bool,
    #[graphql(name = "beginDate")]
    pub begin_date: Option<PartialDate>,
    #[graphql(name = "endDate")]
    pub end_date: Option<PartialDate>,

    #[graphql(skip)]
    pub id: i32,
}

impl From<LabelRow> for Label {
    fn from(r: LabelRow) -> Self {
        Self {
            mbid: r.gid,
            name: r.name,
            disambiguation: r.comment,
            label_type: r.label_type,
            ended: r.ended,
            begin_date: PartialDate::from_parts(
                r.begin_date_year,
                r.begin_date_month,
                r.begin_date_day,
            ),
            end_date: PartialDate::from_parts(r.end_date_year, r.end_date_month, r.end_date_day),
            id: r.id,
        }
    }
}

#[derive(Default)]
pub struct LabelQuery;

#[Object]
impl LabelQuery {
    async fn label(&self, ctx: &Context<'_>, mbid: String) -> async_graphql::Result<Option<Label>> {
        let pool = ctx.data::<PgPool>()?;
        let uuid = Uuid::parse_str(&mbid)?;

        let row = sqlx::query_as::<_, LabelRow>(
            "SELECT
                id,
                gid,
                name,
                begin_date_year,
                begin_date_month,
                begin_date_day,
                end_date_year,
                end_date_month,
                end_date_day,
                area,
                type,
                comment,
                ended,
                label_code
            FROM label
            WHERE gid = $1",
        )
        .bind(uuid)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(Label::from))
    }

    async fn labels(
        &self,
        ctx: &Context<'_>,
        mbids: Vec<String>,
    ) -> async_graphql::Result<Vec<Label>> {
        let pool = ctx.data::<PgPool>()?;
        let uuids: Vec<Uuid> = mbids
            .iter()
            .map(|s| Uuid::parse_str(s))
            .collect::<Result<_, _>>()?;

        let rows = sqlx::query_as::<_, LabelRow>(
            "SELECT
                id,
                gid,
                name,
                begin_date_year,
                begin_date_month,
                begin_date_day,
                end_date_year,
                end_date_month,
                end_date_day,
                type,
                comment,
                ended,
                label_code
            FROM label
            WHERE gid = ANY($1)",
        )
        .bind(&uuids)
        .fetch_all(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(rows.into_iter().map(Label::from).collect())
    }
}

#[ComplexObject]
impl Label {
    async fn release(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Release>> {
        info!(label_id = self.id, "Label.releases resolver called");

        let release_id_loader = ctx.data::<DataLoader<ReleaseIdsByLabelLoader>>()?;
        let release_id = release_id_loader
            .load_one(self.id)
            .await?
            .unwrap_or_default();
        info!(
            label_id = self.id,
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
    async fn rating(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Rating>> {
        info!(label_id = self.id, "Label.rating resolver called");
        let loader = ctx.data::<DataLoader<LabelRatingLoader>>()?;
        Ok(loader.load_one(self.id).await?)
    }
    async fn genres(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Genre>> {
        info!(label_id = self.id, "Label.genres resolver called");

        let id_loader = ctx.data::<DataLoader<GenreIdsByLabelLoader>>()?;
        let ids = id_loader.load_one(self.id).await?.unwrap_or_default();

        if ids.is_empty() {
            return Ok(vec![]);
        }

        let genre_loader = ctx.data::<DataLoader<GenreLoader>>()?;
        let genre_map = genre_loader.load_many(ids.clone()).await?;

        Ok(ids
            .into_iter()
            .filter_map(|id| genre_map.get(&id).cloned())
            .collect())
    }
    async fn annotation(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<String>> {
        info!(label_id = self.id, "Label.annotation resolver called");
        let loader = ctx.data::<DataLoader<LabelAnnotationLoader>>()?;
        Ok(loader.load_one(self.id).await?)
    }
    async fn area(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Area>> {
        info!(label_id = self.id, "Label.area resolver called");

        let id_loader = ctx.data::<DataLoader<AreaIdByLabelLoader>>()?;

        let Some(area_id) = id_loader.load_one(self.id).await?.flatten() else {
            return Ok(None);
        };

        let area_loader = ctx.data::<DataLoader<AreaLoader>>()?;

        Ok(area_loader.load_one(area_id).await?)
    }
}
