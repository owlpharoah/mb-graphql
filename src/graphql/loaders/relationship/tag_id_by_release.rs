use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

use crate::graphql::types::common::TagRef;

#[derive(sqlx::FromRow)]
struct ReleaseTagRow {
    release: i32,
    tag: i32,
    count: i32,
}

pub struct TagIdsByReleaseLoader {
    pub pool: PgPool,
}

impl Loader<i32> for TagIdsByReleaseLoader {
    type Value = Vec<TagRef>;
    type Error = async_graphql::Error;

    async fn load(&self, release_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = release_ids.len(),
            "TagIdsByReleaseLoader batch load"
        );

        let rows = sqlx::query_as!(
            ReleaseTagRow,
            "SELECT release, tag, count FROM release_tag WHERE release = ANY($1)",
            release_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "TagIdsByReleaseLoader query returned");

        let mut result: HashMap<i32, Vec<TagRef>> = HashMap::new();
        for row in rows {
            result.entry(row.release).or_default().push(TagRef {
                tag_id: row.tag,
                count: row.count,
            });
        }
        for id in release_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
