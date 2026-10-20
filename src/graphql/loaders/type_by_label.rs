use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct LabelTypeRow {
    label: i32,
    #[sqlx(rename = "type")]
    type_name: String,
}

pub struct LabelTypeLoader {
    pub pool: PgPool,
}

impl Loader<i32> for LabelTypeLoader {
    type Value = Option<String>;
    type Error = async_graphql::Error;

    async fn load(&self, label_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = label_ids.len(), "LabelTypeLoader batch load");

        let rows = sqlx::query_as!(
            LabelTypeRow,
            r#"
            SELECT l.id AS label, lt.name AS type_name
            FROM label l
            JOIN label_type lt ON lt.id = l.type
            WHERE l.id = ANY($1)
            "#,
            label_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "LabelTypeLoader query returned");

        let mut result: HashMap<i32, Option<String>> = HashMap::new();
        for row in rows {
            result.insert(row.label, Some(row.type_name));
        }
        for id in label_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
