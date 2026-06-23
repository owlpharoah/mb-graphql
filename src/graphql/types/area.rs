use crate::graphql::{
    loaders::{
        entity::tag::TagLoader, iso_code_1_by_area::IsoCode1ByAreaLoader,
        iso_code_2_by_area::IsoCode2ByAreaLoader, iso_code_3_by_area::IsoCode3ByAreaLoader,
        relationship::tag_id_by_area::TagIdsByAreaLoader,
    },
    types::common::{PartialDate, Tag},
};
use async_graphql::{ComplexObject, Context, Object, SimpleObject, dataloader::DataLoader};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct AreaRow {
    id: i32,
    gid: Uuid,
    name: String,
    comment: Option<String>,
    #[sqlx(rename = "type")]
    area_type: Option<i32>,
    ended: bool,
    begin_date_year: Option<i16>,
    begin_date_month: Option<i16>,
    begin_date_day: Option<i16>,
    end_date_year: Option<i16>,
    end_date_month: Option<i16>,
    end_date_day: Option<i16>,
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
    async fn area(&self, ctx: &Context<'_>, mbid: String) -> async_graphql::Result<Option<Area>> {
        let pool = ctx.data::<PgPool>()?;
        let uuid = Uuid::parse_str(&mbid)?;

        let row = sqlx::query_as::<_, AreaRow>(
            "SELECT
                id,
                gid,
                name,
                comment,
                type,
                ended,
                begin_date_year,
                begin_date_month,
                begin_date_day,
                end_date_year,
                end_date_month,
                end_date_day
            FROM area
            WHERE gid = $1;",
        )
        .bind(uuid)
        .fetch_optional(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(row.map(Area::from))
    }

    async fn areas(
        &self,
        ctx: &Context<'_>,
        mbids: Vec<String>,
    ) -> async_graphql::Result<Vec<Area>> {
        let pool = ctx.data::<PgPool>()?;
        let uuids: Vec<Uuid> = mbids
            .iter()
            .map(|s| Uuid::parse_str(s))
            .collect::<Result<_, _>>()?;

        let rows = sqlx::query_as::<_, AreaRow>(
            "SELECT
                id,
                gid,
                name,
                comment,
                type,
                ended,
                begin_date_year,
                begin_date_month,
                begin_date_day,
                end_date_year,
                end_date_month,
                end_date_day
            FROM area
            WHERE gid = ANY($1);",
        )
        .bind(&uuids)
        .fetch_all(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(rows.into_iter().map(Area::from).collect())
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
}
