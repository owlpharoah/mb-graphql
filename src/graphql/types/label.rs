use async_graphql::{ComplexObject, Context, Object, SimpleObject};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::graphql::types::{self};
use types::common::PartialDate;

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
    pub area: Option<i32>,
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
    #[graphql(skip)]
    pub area: Option<i32>,
}

impl From<LabelRow> for Label {
    fn from(r: LabelRow) -> Self {
        Self {
            mbid: r.gid,
            name: r.name,
            disambiguation: r.comment,
            area: r.area,
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
                area,
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
    async fn first_release_date(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Option<PartialDate>> {
        todo!()
    }
}
