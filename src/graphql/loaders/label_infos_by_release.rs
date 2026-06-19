use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;

use crate::graphql::types::common::LabelInfo;

#[derive(sqlx::FromRow)]
struct ReleaseLabelInfoRow {
    label: Option<i32>,
    catalog_number: Option<String>,
    release: i32,
}

pub struct LabelInfosByReleaseLoader {
    pub pool: PgPool,
}

impl Loader<i32> for LabelInfosByReleaseLoader {
    type Value = Vec<LabelInfo>;
    type Error = async_graphql::Error;

    async fn load(&self, release_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let rows = sqlx::query_as::<_, ReleaseLabelInfoRow>(
            "SELECT release, label , catalog_number FROM release_label WHERE release = ANY($1)",
        )
        .bind(release_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let mut result: HashMap<i32, Vec<LabelInfo>> = HashMap::new();
        for row in rows {
            result.entry(row.release).or_default().push(LabelInfo {
                catalog_number: row.catalog_number,
                label_id: row.label,
            });
        }
        for id in release_ids {
            result.entry(*id).or_default();
        }
        Ok(result)
    }
}
