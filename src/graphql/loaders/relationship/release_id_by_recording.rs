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

use crate::graphql::loaders::relationship::PageKey;
impl Loader<PageKey> for ReleaseIdsByRecordingLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[PageKey]) -> Result<HashMap<PageKey, Self::Value>, Self::Error> {
        info!(count = keys.len(), "ReleaseIdsByRecordingLoader batch load");

        let mut groups: HashMap<(Option<i32>, i32), Vec<i32>> = HashMap::new();
        for key in keys {
            groups
                .entry((key.after, key.first))
                .or_default()
                .push(key.entity_id);
        }

        let mut result: HashMap<PageKey, Vec<i32>> = HashMap::new();

        for ((after, first), entity_ids) in groups {
            let rows = sqlx::query_as!(
                RecordingReleaseIdRow,
                r#"SELECT recording AS "recording!", release AS "release!"
                FROM (
                    SELECT t.recording, m.release,
                           ROW_NUMBER() OVER (PARTITION BY t.recording ORDER BY m.release) AS rn
                    FROM track t
                    JOIN medium m ON m.id = t.medium
                    WHERE t.recording = ANY($1)
                      AND ($2::int IS NULL OR m.release > $2)
                ) ranked
                WHERE rn <= $3
                ORDER BY recording, release"#,
                &entity_ids,
                after,
                first as i64
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

            let mut group_result: HashMap<i32, Vec<i32>> = HashMap::new();
            for row in rows {
                group_result
                    .entry(row.recording)
                    .or_default()
                    .push(row.release);
            }

            for id in &entity_ids {
                result.insert(
                    PageKey {
                        entity_id: *id,
                        after,
                        first,
                    },
                    group_result.remove(id).unwrap_or_default(),
                );
            }
        }

        Ok(result)
    }
}
