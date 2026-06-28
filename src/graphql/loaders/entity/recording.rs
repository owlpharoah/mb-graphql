use crate::graphql::types::recording::{Recording, RecordingRow};
use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

pub struct RecordingLoader {
    pub pool: PgPool,
}

impl Loader<i32> for RecordingLoader {
    type Value = Recording;
    type Error = async_graphql::Error;

    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = ids.len(), "RecordingLoader batch load");
        let rows = sqlx::query_as!(
            RecordingRow,
            r#"SELECT
                id,
                gid,
                name,
                comment,
                length,
                video
            FROM recording
            WHERE id = ANY($1)"#,
            ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(rows = rows.len(), "RecordingLoader query returned");

        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    row.id,
                    Recording {
                        mbid: row.gid,
                        name: row.name,
                        disambiguation: row.comment,
                        length: row.length,
                        video: row.video,
                        id: row.id,
                    },
                )
            })
            .collect())
    }
}
