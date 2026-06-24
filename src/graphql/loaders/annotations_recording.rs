use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct RecordingAnnotationRow {
    recording: i32,
    text: Option<String>,
}

pub struct RecordingAnnotationLoader {
    pub pool: PgPool,
}

impl Loader<i32> for RecordingAnnotationLoader {
    type Value = String;
    type Error = async_graphql::Error;

    async fn load(&self, recording_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = recording_ids.len(),
            "RecordingAnnotationLoader batch load"
        );

        let rows = sqlx::query_as::<_, RecordingAnnotationRow>(
            r#"SELECT rca.recording AS recording, a.text AS text
            FROM recording_annotation rca
            JOIN annotation a ON a.id = rca.annotation
            WHERE rca.recording = ANY($1)"#,
        )
        .bind(recording_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(
            rows = rows.len(),
            "RecordingAnnotationLoader query returned"
        );

        Ok(rows
            .into_iter()
            .filter_map(|row| row.text.map(|text| (row.recording, text)))
            .collect())
    }
}
