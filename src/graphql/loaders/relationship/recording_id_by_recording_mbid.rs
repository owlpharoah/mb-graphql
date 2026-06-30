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
#[derive(sqlx::FromRow)]
struct RecordingRedirectRow {
    gid: Uuid,
    new_id: i32,
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
        let rows = sqlx::query_as!(
            RecordingIDMBIDRow,
            "SELECT gid, id FROM recording WHERE gid = ANY($1)",
            recording_mbids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(rows = rows.len(), "RecordingIDByMBIDLoader query returned");
        let mut result: HashMap<Uuid, i32> = HashMap::new();
        for row in rows {
            result.insert(row.gid, row.id);
        }
        let unresolved: Vec<Uuid> = recording_mbids
            .iter()
            .filter(|gid| !result.contains_key(gid))
            .copied()
            .collect();

        if !unresolved.is_empty() {
            let redirects = sqlx::query_as!(
                RecordingRedirectRow,
                "SELECT gid, new_id FROM recording_gid_redirect WHERE gid = ANY($1)",
                &unresolved
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

            info!(
                redirects = redirects.len(),
                "RecordingIDByMBIDLoader redirect lookup returned"
            );

            for row in redirects {
                result.insert(row.gid, row.new_id);
            }
        }
        Ok(result)
    }
}
