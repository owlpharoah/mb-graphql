use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;

use crate::graphql::types::recording::{Recording, RecordingRow};

pub struct RecordingLoader {
    pub pool: PgPool,
}

impl Loader<i32> for RecordingLoader {
    type Value = Recording;
    type Error = async_graphql::Error;

    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let rows = sqlx::query_as::<_, RecordingRow>(
            r#"SELECT
                id,
                gid,
                name,
                artist_credit,
                comment,
                length,
                video
            FROM recording
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
                    Recording {
                        mbid: row.gid,
                        name: row.name,
                        disambiguation: row.comment,
                        length: row.length,
                        video: row.video,
                        id: row.id,
                        artist_credit: row.artist_credit,
                    },
                )
            })
            .collect())
    }
}
