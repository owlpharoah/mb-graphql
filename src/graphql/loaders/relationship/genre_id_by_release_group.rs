use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct ReleaseGroupGenreIdRow {
    release_group: i32,
    id: i32,
}

pub struct GenreIdsByReleaseGroupLoader {
    pub pool: PgPool,
}

use crate::graphql::loaders::relationship::PageKey;

impl Loader<PageKey> for GenreIdsByReleaseGroupLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[PageKey]) -> Result<HashMap<PageKey, Self::Value>, Self::Error> {
        info!(
            count = keys.len(),
            "GenreIdsByReleaseGroupLoader batch load"
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
                ReleaseGroupGenreIdRow,
                r#"SELECT release_group AS "release_group!", id AS "id!"
                FROM (
                    SELECT rgt.release_group, g.id,
                           ROW_NUMBER() OVER (PARTITION BY rgt.release_group ORDER BY g.id) AS rn
                    FROM release_group_tag rgt
                    JOIN tag t ON t.id = rgt.tag
                    JOIN genre g ON g.name = t.name
                    WHERE rgt.release_group = ANY($1)
                      AND ($2::int IS NULL OR g.id > $2)
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
