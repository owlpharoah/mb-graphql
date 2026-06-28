use crate::graphql::{
    loaders::{
        alias_area::AreaAliasLoader,
        annotations_area::AreaAnnotationLoader,
        entity::{area::AreaLoader, tag::TagLoader},
        iso_code_1_by_area::IsoCode1ByAreaLoader,
        iso_code_2_by_area::IsoCode2ByAreaLoader,
        iso_code_3_by_area::IsoCode3ByAreaLoader,
        relationship::{
            area_id_by_area_mbid::AreaIDByMBIDLoader, tag_id_by_area::TagIdsByAreaLoader,
        },
    },
    types::common::{Alias, PartialDate, Tag},
};
use async_graphql::{ComplexObject, Context, Object, SimpleObject, dataloader::DataLoader};
use serde::{Deserialize, Serialize};

use tracing::info;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct AreaRow {
    pub id: i32,
    pub gid: Uuid,
    pub name: String,
    pub comment: Option<String>,
    #[sqlx(rename = "type")]
    pub area_type: Option<i32>,
    pub ended: bool,
    pub begin_date_year: Option<i16>,
    pub begin_date_month: Option<i16>,
    pub begin_date_day: Option<i16>,
    pub end_date_year: Option<i16>,
    pub end_date_month: Option<i16>,
    pub end_date_day: Option<i16>,
}

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Area {
    pub mbid: Uuid,
    pub name: String,
    pub disambiguation: Option<String>,
    #[graphql(name = "type")]
    pub area_type: Option<i32>,
    pub ended: bool,
    #[graphql(name = "beginDate")]
    pub begin_date: Option<PartialDate>,
    #[graphql(name = "endDate")]
    pub end_date: Option<PartialDate>,

    #[graphql(skip)]
    pub id: i32,
}

impl From<AreaRow> for Area {
    fn from(r: AreaRow) -> Self {
        Self {
            mbid: r.gid,
            name: r.name,
            area_type: r.area_type,
            disambiguation: r.comment,
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
pub struct AreaQuery;

#[Object]
impl AreaQuery {
    async fn area(&self, ctx: &Context<'_>, mbid: Vec<String>) -> async_graphql::Result<Vec<Area>> {
        let uuids: Vec<Uuid> = mbid
            .iter()
            .map(|s| Uuid::parse_str(s))
            .collect::<Result<_, _>>()?;
        let id_loader = ctx.data::<DataLoader<AreaIDByMBIDLoader>>()?;
        let ids = id_loader.load_many(uuids).await?;
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let area_loader = ctx.data::<DataLoader<AreaLoader>>()?;
        let area_ids: Vec<i32> = ids.values().copied().collect();
        let areas_map = area_loader.load_many(area_ids).await?;
        Ok(ids
            .into_iter()
            .filter_map(|(_uuid, id)| areas_map.get(&id).cloned())
            .collect())
    }
}

#[ComplexObject]
impl Area {
    async fn iso_code_1(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<String>> {
        let loader = ctx.data::<DataLoader<IsoCode1ByAreaLoader>>()?;

        Ok(loader.load_one(self.id).await?.unwrap_or_default())
    }
    async fn iso_code_2(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<String>> {
        let loader = ctx.data::<DataLoader<IsoCode2ByAreaLoader>>()?;

        Ok(loader.load_one(self.id).await?.unwrap_or_default())
    }
    async fn iso_code_3(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<String>> {
        let loader = ctx.data::<DataLoader<IsoCode3ByAreaLoader>>()?;

        Ok(loader.load_one(self.id).await?.unwrap_or_default())
    }
    async fn tags(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Tag>> {
        info!(area_id = self.id, "Area.tags resolver called");

        let id_loader = ctx.data::<DataLoader<TagIdsByAreaLoader>>()?;
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
    async fn annotation(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<String>> {
        info!(area_id = self.id, "Area.annotation resolver called");
        let loader = ctx.data::<DataLoader<AreaAnnotationLoader>>()?;
        Ok(loader.load_one(self.id).await?)
    }
    async fn alias(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Alias>> {
        info!(area_id = self.id, "Area.aliases resolver called");
        let loader = ctx.data::<DataLoader<AreaAliasLoader>>()?;
        Ok(loader.load_one(self.id).await?.unwrap_or_default())
    }
}
