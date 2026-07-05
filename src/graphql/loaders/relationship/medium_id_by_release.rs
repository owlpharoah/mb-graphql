use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;
#[derive(sqlx::FromRow)]
struct ReleaseMediumIdRow {
    id: i32,
    release: i32,
}

pub struct MediumIdByReleaseLoader {
    pub pool: PgPool,
}

use crate::graphql::loaders::relationship::PageKey;

impl Loader<PageKey> for MediumIdByReleaseLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[PageKey]) -> Result<HashMap<PageKey, Self::Value>, Self::Error> {
        info!(count = keys.len(), "MediumIdByReleaseLoader batch load");

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
                ReleaseMediumIdRow,
                r#"SELECT release AS "release!", id AS "id!"
                FROM (
                    SELECT release, id,
                           ROW_NUMBER() OVER (PARTITION BY release ORDER BY id) AS rn
                    FROM medium
                    WHERE release = ANY($1)
                      AND ($2::int IS NULL OR id > $2)
                ) ranked
                WHERE rn <= $3
                ORDER BY release, id"#,
                &entity_ids,
                after,
                first as i64
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

            let mut group_result: HashMap<i32, Vec<i32>> = HashMap::new();
            for row in rows {
                group_result.entry(row.release).or_default().push(row.id);
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
