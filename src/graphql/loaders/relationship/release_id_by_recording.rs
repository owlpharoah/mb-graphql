use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;

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
        let rows = sqlx::query_as::<_, RecordingReleaseIdRow>(
            "SELECT DISTINCT
                t.recording,
                m.release
            FROM track t
            JOIN medium m ON m.id = t.medium
            WHERE t.recording = ANY($1);",
        )
        .bind(recording_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

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
