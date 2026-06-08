use async_graphql::{ComplexObject, Context, Object, SimpleObject};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::graphql::types;
use types::common::PartialDate;

#[derive(sqlx::FromRow)]
struct ReleaseGroupRow {
    id: i32,
    gid: Uuid,
    name: String,
    artist_credit: i32,
    comment: Option<String>,
    #[sqlx(rename = "type")]
    release_group_type: Option<i16>,
}
//TODO: TO make sure you can access stuff like primary type and stuff direcly by queruing rg as well as by artist
#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct ReleaseGroup {
    pub mbid: Uuid,
    pub name: String,
    pub disambiguation: Option<String>,
    #[graphql(name = "type")]
    pub release_group_type: Option<i16>,

    #[graphql(skip)]
    pub id: i32,
    #[graphql(skip)]
    pub artist_credit: i32,
    #[graphql(skip)]
    pub secondary_types: Option<Vec<i16>>,
    pub first_release_date: Option<PartialDate>,
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
            secondary_types: Some(vec![]),
            first_release_date: None,
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
}

#[ComplexObject]
impl ReleaseGroup {
    async fn area(&self) -> String {
        todo!()
    }
}
