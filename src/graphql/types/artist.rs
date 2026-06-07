use async_graphql::{ComplexObject, Context, Object, ScalarType, SimpleObject};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::graphql::types;
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
    pub(crate) id: i32,
    #[graphql(skip)]
    pub area: Option<i32>,
    #[graphql(skip)]
    pub(crate) begin_area: Option<i32>,
    #[graphql(skip)]
    pub(crate) end_area: Option<i32>,
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
        .await?;

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
            .map(|s| Uuid::parse_str(&s))
            .collect::<Result<_, _>>()?;

        let rows = sqlx::query_as::<_, ArtistRow>(
            "SELECT id, gid, name, sort_name, comment,type, gender, area, ended, begin_date_year, begin_date_month, begin_date_day, end_date_year, end_date_month, end_date_day,begin_area,end_area
                        FROM artist
                        WHERE gid = ANY($1)",
        )
        .bind(&uuids)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(Artist::from).collect())
    }
}

#[ComplexObject]
impl Artist {
    async fn game(&self) -> String {
        todo!()
    }
}
