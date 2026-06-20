use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use crate::graphql::types::common::Medium;

pub struct MediumLoader {
    pub pool: PgPool,
}

#[derive(sqlx::FromRow)]
pub struct MediumRow {
    id: i32,
    gid: Uuid,
    name: String,
    release: i32,
    position: i32,
    format: Option<i32>,
    track_count: i32,
}

impl Loader<i32> for MediumLoader {
    type Value = Medium;
    type Error = async_graphql::Error;

    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let rows = sqlx::query_as::<_, MediumRow>(
            r#"SELECT
                id,
                gid,
                name,
                release,
                position,
                format,
                track_count
            FROM medium
            WHERE id = ANY($1)"#,
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
                    Medium {
                        mbid: row.gid,
                        name: row.name,
                        format: row.format,
                        position: row.position,
                        track_count: row.track_count,
                        id: row.id,
                        release: row.release,
                    },
                )
            })
            .collect())
    }
}
