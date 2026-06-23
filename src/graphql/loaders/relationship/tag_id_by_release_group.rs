use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

use crate::graphql::types::common::TagRef;

#[derive(sqlx::FromRow)]
struct ReleaseGroupTagRow {
    release_group: i32,
    tag: i32,
    count: i32,
}

pub struct TagIdsByReleaseGroupLoader {
    pub pool: PgPool,
}

impl Loader<i32> for TagIdsByReleaseGroupLoader {
    type Value = Vec<TagRef>;
    type Error = async_graphql::Error;

    async fn load(
        &self,
        release_group_ids: &[i32],
    ) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = release_group_ids.len(),
            "TagIdsByReleaseGroupLoader batch load"
        );

        let rows = sqlx::query_as::<_, ReleaseGroupTagRow>(
            "SELECT release_group, tag, count FROM release_group_tag WHERE release_group = ANY($1)",
        )
        .bind(release_group_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(
            rows = rows.len(),
            "TagIdsByReleaseGroupLoader query returned"
        );

        let mut result: HashMap<i32, Vec<TagRef>> = HashMap::new();
        for row in rows {
            result.entry(row.release_group).or_default().push(TagRef {
                tag_id: row.tag,
                count: row.count,
            });
        }
        for id in release_group_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
