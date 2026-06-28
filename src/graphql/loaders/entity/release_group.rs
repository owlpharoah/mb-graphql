use crate::graphql::types::release_group::ReleaseGroup;
use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct ReleaseGroupRow {
    id: i32,
    gid: Uuid,
    name: String,
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
        info!(count = ids.len(), "ReleaseGroupLoader batch load");
        let rows = sqlx::query_as!(
            ReleaseGroupRow,
            r#"SELECT id, gid, name, comment, type AS primary_type
               FROM release_group WHERE id = ANY($1)"#,
            ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(rows = rows.len(), "ReleaseGroupLoader query returned");

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
                    },
                )
            })
            .collect())
    }
}
