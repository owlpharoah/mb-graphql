use crate::graphql::{
    loaders::{
        alias_label::LabelAliasLoader,
        annotations_label::LabelAnnotationLoader,
        entity::{
            area::AreaLoader, genre::GenreLoader, label::LabelLoader, release::ReleaseLoader,
        },
        label_ipi::LabelIpiLoader,
        label_isni::LabelIsniLoader,
        rating_label::LabelRatingLoader,
        relationship::{
            area_id_by_label::AreaIdByLabelLoader, genre_id_by_label::GenreIdsByLabelLoader,
            label_id_by_label_mbid::LabelIDByMBIDLoader,
            release_id_by_label::ReleaseIdsByLabelLoader,
        },
    },
    types::{
        self,
        area::Area,
        common::{Alias, Genre, Rating},
        release::Release,
    },
};
use async_graphql::{ComplexObject, Context, Object, SimpleObject, dataloader::DataLoader};
use serde::{Deserialize, Serialize};
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
    async fn label(
        &self,
        ctx: &Context<'_>,
        mbid: Vec<String>,
    ) -> async_graphql::Result<Vec<Label>> {
        let uuids: Vec<Uuid> = mbid
            .iter()
            .map(|s| Uuid::parse_str(s))
            .collect::<Result<_, _>>()?;
        let id_loader = ctx.data::<DataLoader<LabelIDByMBIDLoader>>()?;
        let ids = id_loader.load_many(uuids).await?;
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let label_loader = ctx.data::<DataLoader<LabelLoader>>()?;
        let label_ids: Vec<i32> = ids.values().copied().collect();
        let labels_map = label_loader.load_many(label_ids).await?;
        Ok(ids
            .into_iter()
            .filter_map(|(_uuid, id)| labels_map.get(&id).cloned())
            .collect())
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
        loader.load_one(self.id).await
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
        loader.load_one(self.id).await
    }
    async fn area(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<Area>> {
        info!(label_id = self.id, "Label.area resolver called");

        let id_loader = ctx.data::<DataLoader<AreaIdByLabelLoader>>()?;

        let Some(area_id) = id_loader.load_one(self.id).await?.flatten() else {
            return Ok(None);
        };

        let area_loader = ctx.data::<DataLoader<AreaLoader>>()?;

        area_loader.load_one(area_id).await
    }
    async fn ipis(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<String>> {
        info!(label_id = self.id, "Label.ipis resolver called");
        let loader = ctx.data::<DataLoader<LabelIpiLoader>>()?;
        Ok(loader.load_one(self.id).await?.unwrap_or_default())
    }

    async fn isnis(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<String>> {
        info!(label_id = self.id, "Label.isnis resolver called");
        let loader = ctx.data::<DataLoader<LabelIsniLoader>>()?;
        Ok(loader.load_one(self.id).await?.unwrap_or_default())
    }
    async fn alias(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Alias>> {
        info!(label_id = self.id, "Label.aliases resolver called");
        let loader = ctx.data::<DataLoader<LabelAliasLoader>>()?;
        Ok(loader.load_one(self.id).await?.unwrap_or_default())
    }
}
