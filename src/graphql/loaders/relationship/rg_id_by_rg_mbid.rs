use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct ReleaseGroupIDMBIDRow {
    gid: Uuid,
    id: i32,
}

pub struct ReleaseGroupIDByMBIDLoader {
    pub pool: PgPool,
}

impl Loader<Uuid> for ReleaseGroupIDByMBIDLoader {
    type Value = i32;
    type Error = async_graphql::Error;

    async fn load(
        &self,
        release_group_mbids: &[Uuid],
    ) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        info!(
            count = release_group_mbids.len(),
            "ReleaseGroupIDByMBIDLoader batch load"
        );
        let rows = sqlx::query_as!(
            ReleaseGroupIDMBIDRow,
            "SELECT gid, id FROM release_group WHERE gid = ANY($1)",
            release_group_mbids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(
            rows = rows.len(),
            "ReleaseGroupIDByMBIDLoader query returned"
        );
        let mut result: HashMap<Uuid, i32> = HashMap::new();
        for row in rows {
            result.insert(row.gid, row.id);
        }
        Ok(result)
    }
}
