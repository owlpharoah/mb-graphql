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

impl Loader<i32> for GenreIdsByLabelLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, label_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = label_ids.len(), "GenreIdsByLabelLoader batch load");

        let rows = sqlx::query_as!(
            LabelGenreIdRow,
            r#"SELECT lt.label AS label, g.id AS id
            FROM label_tag lt
            JOIN tag t ON t.id = lt.tag
            JOIN genre g ON g.name = t.name
            WHERE lt.label = ANY($1)
            ORDER BY lt.label, lt.count DESC"#,
            label_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "GenreIdsByLabelLoader query returned");

        let mut result: HashMap<i32, Vec<i32>> = HashMap::new();
        for row in rows {
            result.entry(row.label).or_default().push(row.id);
        }
        for id in label_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
