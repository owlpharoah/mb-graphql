use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct RecordingISRCRow {
    recording: i32,
    isrc: String,
}

pub struct RecordingISRCLoader {
    pub pool: PgPool,
}

impl Loader<i32> for RecordingISRCLoader {
    type Value = Vec<String>;
    type Error = async_graphql::Error;

    async fn load(&self, recording_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = recording_ids.len(),
            "RecordingISRCLoader batch load"
        );

        let rows = sqlx::query_as!(
            RecordingISRCRow,
            "SELECT recording, isrc FROM isrc WHERE recording = ANY($1)",
            recording_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "RecordingISRCLoader query returned");

        let mut result: HashMap<i32, Vec<String>> = HashMap::new();
        for row in rows {
            result.entry(row.recording).or_default().push(row.isrc);
        }
        for id in recording_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
