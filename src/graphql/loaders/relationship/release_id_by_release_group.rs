use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;
#[derive(sqlx::FromRow)]
struct ReleaseGroupReleaseIdRow {
    id: i32,
    release_group: i32,
}

pub struct ReleaseIdByReleaseGroupLoader {
    pub pool: PgPool,
}

use crate::graphql::loaders::relationship::PageKey;
impl Loader<PageKey> for ReleaseIdByReleaseGroupLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[PageKey]) -> Result<HashMap<PageKey, Self::Value>, Self::Error> {
        info!(
            count = keys.len(),
            "ReleaseIdByReleaseGroupLoader batch load"
        );

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
                ReleaseGroupReleaseIdRow,
                r#"SELECT release_group AS "release_group!", id AS "id!"
                FROM (
                    SELECT release_group, id,
                           ROW_NUMBER() OVER (PARTITION BY release_group ORDER BY id) AS rn
                    FROM release
                    WHERE release_group = ANY($1)
                      AND ($2::int IS NULL OR id > $2)
                ) ranked
                WHERE rn <= $3
                ORDER BY release_group, id"#,
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
                    .entry(row.release_group)
                    .or_default()
                    .push(row.id);
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
