use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

use crate::graphql::types::common::TagRef;

#[derive(sqlx::FromRow)]
struct LabelTagRow {
    label: i32,
    tag: i32,
    count: i32,
}

pub struct TagIdsByLabelLoader {
    pub pool: PgPool,
}

impl Loader<i32> for TagIdsByLabelLoader {
    type Value = Vec<TagRef>;
    type Error = async_graphql::Error;

    async fn load(&self, label_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = label_ids.len(), "TagIdsByLabelLoader batch load");

        let rows = sqlx::query_as::<_, LabelTagRow>(
            "SELECT label, tag, count FROM label_tag WHERE label = ANY($1)",
        )
        .bind(label_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "TagIdsByLabelLoader query returned");

        let mut result: HashMap<i32, Vec<TagRef>> = HashMap::new();
        for row in rows {
            result.entry(row.label).or_default().push(TagRef {
                tag_id: row.tag,
                count: row.count,
            });
        }
        for id in label_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
