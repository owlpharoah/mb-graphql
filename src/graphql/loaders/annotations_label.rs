use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct LabelAnnotationRow {
    label: i32,
    text: Option<String>,
}

pub struct LabelAnnotationLoader {
    pub pool: PgPool,
}

impl Loader<i32> for LabelAnnotationLoader {
    type Value = String;
    type Error = async_graphql::Error;

    async fn load(&self, label_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = label_ids.len(), "LabelAnnotationLoader batch load");

        let rows = sqlx::query_as::<_, LabelAnnotationRow>(
            r#"SELECT la.label AS label, a.text AS text
            FROM label_annotation la
            JOIN annotation a ON a.id = la.annotation
            WHERE la.label = ANY($1)"#,
        )
        .bind(label_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "LabelAnnotationLoader query returned");

        Ok(rows
            .into_iter()
            .filter_map(|row| row.text.map(|text| (row.label, text)))
            .collect())
    }
}
