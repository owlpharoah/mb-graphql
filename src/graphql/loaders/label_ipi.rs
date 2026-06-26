use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct LabelIpiRow {
    label: i32,
    ipi: String,
}

pub struct LabelIpiLoader {
    pub pool: PgPool,
}

impl Loader<i32> for LabelIpiLoader {
    type Value = Vec<String>;
    type Error = async_graphql::Error;

    async fn load(&self, label_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = label_ids.len(), "LabelIpiLoader batch load");

        let rows = sqlx::query_as::<_, LabelIpiRow>(
            "SELECT label, ipi FROM label_ipi WHERE label = ANY($1)",
        )
        .bind(label_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "LabelIpiLoader query returned");

        let mut result: HashMap<i32, Vec<String>> = HashMap::new();
        for row in rows {
            result.entry(row.label).or_default().push(row.ipi);
        }
        for id in label_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
