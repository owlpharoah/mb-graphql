use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct MediumFormatRow {
    medium: i32,
    #[sqlx(rename = "type")]
    type_name: String,
}

pub struct MediumFormatLoader {
    pub pool: PgPool,
}

impl Loader<i32> for MediumFormatLoader {
    type Value = Option<String>;
    type Error = async_graphql::Error;

    async fn load(&self, medium_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = medium_ids.len(), "MediumFormatLoader batch load");

        let rows = sqlx::query_as!(
            MediumFormatRow,
            r#"
            SELECT m.id AS medium, mf.name AS type_name
            FROM medium m
            JOIN medium_format mf ON mf.id = m.format
            WHERE m.id = ANY($1)
            "#,
            medium_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "MediumFormatLoader query returned");

        let mut result: HashMap<i32, Option<String>> = HashMap::new();
        for row in rows {
            result.insert(row.medium, Some(row.type_name));
        }
        for id in medium_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
