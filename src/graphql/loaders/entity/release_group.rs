use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use crate::graphql::types::release_group::ReleaseGroup;

#[derive(sqlx::FromRow)]
struct ReleaseGroupRow {
    id: i32,
    gid: Uuid,
    name: String,
    artist_credit: i32,
    comment: Option<String>,
    #[sqlx(rename = "type")]
    primary_type: Option<i32>,
}

pub struct ReleaseGroupLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ReleaseGroupLoader {
    type Value = ReleaseGroup;
    type Error = async_graphql::Error;

    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let rows = sqlx::query_as::<_, ReleaseGroupRow>(
            r#"SELECT id, gid, name, artist_credit, comment, type
               FROM release_group WHERE id = ANY($1)"#,
        )
        .bind(ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    row.id,
                    ReleaseGroup {
                        mbid: row.gid,
                        name: row.name,
                        disambiguation: row.comment,
                        release_group_type: row.primary_type,
                        id: row.id,
                        artist_credit: row.artist_credit,
                    },
                )
            })
            .collect())
    }
}
