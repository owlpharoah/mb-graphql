use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct LabelGenreIdRow {
    label: i32,
    id: i32,
}

pub struct GenreIdsByLabelLoader {
    pub pool: PgPool,
}

use crate::graphql::loaders::relationship::PageKey;

impl Loader<PageKey> for GenreIdsByLabelLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[PageKey]) -> Result<HashMap<PageKey, Self::Value>, Self::Error> {
        info!(count = keys.len(), "GenreIdsByLabelLoader batch load");

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
                LabelGenreIdRow,
                r#"SELECT label AS "label!", id AS "id!"
                FROM (
                    SELECT lt.label, g.id,
                           ROW_NUMBER() OVER (PARTITION BY lt.label ORDER BY g.id) AS rn
                    FROM label_tag lt
                    JOIN tag t ON t.id = lt.tag
                    JOIN genre g ON g.name = t.name
                    WHERE lt.label = ANY($1)
                      AND ($2::int IS NULL OR g.id > $2)
                ) ranked
                WHERE rn <= $3
                ORDER BY label, id"#,
                &entity_ids,
                after,
                first as i64
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

            let mut group_result: HashMap<i32, Vec<i32>> = HashMap::new();
            for row in rows {
                group_result.entry(row.label).or_default().push(row.id);
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
