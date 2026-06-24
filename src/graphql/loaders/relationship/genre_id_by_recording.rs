use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct RecordingGenreIdRow {
    recording: i32,
    id: i32,
}

pub struct GenreIdsByRecordingLoader {
    pub pool: PgPool,
}

impl Loader<i32> for GenreIdsByRecordingLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, recording_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = recording_ids.len(),
            "GenreIdsByRecordingLoader batch load"
        );

        let rows = sqlx::query_as::<_, RecordingGenreIdRow>(
            r#"SELECT rct.recording AS recording, g.id AS id
            FROM recording_tag rct
            JOIN tag t ON t.id = rct.tag
            JOIN genre g ON g.name = t.name
            WHERE rct.recording = ANY($1)
            ORDER BY rct.recording, rct.count DESC"#,
        )
        .bind(recording_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(
            rows = rows.len(),
            "GenreIdsByRecordingLoader query returned"
        );

        let mut result: HashMap<i32, Vec<i32>> = HashMap::new();
        for row in rows {
            result.entry(row.recording).or_default().push(row.id);
        }
        for id in recording_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
