use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

use crate::graphql::types::common::TagRef;

#[derive(sqlx::FromRow)]
struct AreaTagRow {
    area: i32,
    tag: i32,
    count: i32,
}

pub struct TagIdsByAreaLoader {
    pub pool: PgPool,
}

impl Loader<i32> for TagIdsByAreaLoader {
    type Value = Vec<TagRef>;
    type Error = async_graphql::Error;

    async fn load(&self, area_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = area_ids.len(), "TagIdsByAreaLoader batch load");

        let rows = sqlx::query_as::<_, AreaTagRow>(
            "SELECT area, tag, count FROM area_tag WHERE area = ANY($1)",
        )
        .bind(area_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "TagIdsByAreaLoader query returned");

        let mut result: HashMap<i32, Vec<TagRef>> = HashMap::new();
        for row in rows {
            result.entry(row.area).or_default().push(TagRef {
                tag_id: row.tag,
                count: row.count,
            });
        }
        for id in area_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
