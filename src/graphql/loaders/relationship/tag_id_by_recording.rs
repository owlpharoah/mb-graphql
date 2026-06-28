use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

use crate::graphql::types::common::TagRef;

#[derive(sqlx::FromRow)]
struct RecordingTagRow {
    recording: i32,
    tag: i32,
    count: i32,
}

pub struct TagIdsByRecordingLoader {
    pub pool: PgPool,
}

impl Loader<i32> for TagIdsByRecordingLoader {
    type Value = Vec<TagRef>;
    type Error = async_graphql::Error;

    async fn load(&self, recording_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = recording_ids.len(),
            "TagIdsByRecordingLoader batch load"
        );

        let rows = sqlx::query_as!(
            RecordingTagRow,
            "SELECT recording, tag, count FROM recording_tag WHERE recording = ANY($1)",
            recording_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "TagIdsByRecordingLoader query returned");

        let mut result: HashMap<i32, Vec<TagRef>> = HashMap::new();
        for row in rows {
            result.entry(row.recording).or_default().push(TagRef {
                tag_id: row.tag,
                count: row.count,
            });
        }
        for id in recording_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
