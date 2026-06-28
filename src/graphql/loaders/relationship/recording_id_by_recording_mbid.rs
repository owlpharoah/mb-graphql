use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct RecordingIDMBIDRow {
    gid: Uuid,
    id: i32,
}

pub struct RecordingIDByMBIDLoader {
    pub pool: PgPool,
}

impl Loader<Uuid> for RecordingIDByMBIDLoader {
    type Value = i32;
    type Error = async_graphql::Error;

    async fn load(
        &self,
        recording_mbids: &[Uuid],
    ) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        info!(
            count = recording_mbids.len(),
            "RecordingIDByMBIDLoader batch load"
        );
        let rows = sqlx::query_as::<_, RecordingIDMBIDRow>(
            "SELECT gid, id FROM recording WHERE gid = ANY($1)",
        )
        .bind(recording_mbids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(rows = rows.len(), "RecordingIDByMBIDLoader query returned");
        let mut result: HashMap<Uuid, i32> = HashMap::new();
        for row in rows {
            result.insert(row.gid, row.id);
        }
        Ok(result)
    }
}
