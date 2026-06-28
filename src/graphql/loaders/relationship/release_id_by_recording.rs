use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;
#[derive(sqlx::FromRow)]
struct RecordingReleaseIdRow {
    recording: i32,
    release: i32,
}

pub struct ReleaseIdsByRecordingLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ReleaseIdsByRecordingLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, recording_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            ?recording_ids,
            count = recording_ids.len(),
            "ReleaseIdsByRecordingLoader batch load"
        );
        let rows = sqlx::query_as!(
            RecordingReleaseIdRow,
            "SELECT DISTINCT
                t.recording,
                m.release
            FROM track t
            JOIN medium m ON m.id = t.medium
            WHERE t.recording = ANY($1);",
            recording_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(
            rows = rows.len(),
            "ReleaseIdsByRecordingLoader query returned"
        );

        let mut result: HashMap<i32, Vec<i32>> = HashMap::new();
        for row in rows {
            result.entry(row.recording).or_default().push(row.release);
        }
        for id in recording_ids {
            result.entry(*id).or_default();
        }
        Ok(result)
    }
}
