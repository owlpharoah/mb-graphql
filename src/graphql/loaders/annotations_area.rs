use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct AreaAnnotationRow {
    area: i32,
    text: Option<String>,
}

pub struct AreaAnnotationLoader {
    pub pool: PgPool,
}

impl Loader<i32> for AreaAnnotationLoader {
    type Value = String;
    type Error = async_graphql::Error;

    async fn load(&self, area_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = area_ids.len(), "AreaAnnotationLoader batch load");

        let rows = sqlx::query_as!(
            AreaAnnotationRow,
            r#"SELECT aa.area AS area, a.text AS text
            FROM area_annotation aa
            JOIN annotation a ON a.id = aa.annotation
            WHERE aa.area = ANY($1)"#,
            area_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "AreaAnnotationLoader query returned");

        Ok(rows
            .into_iter()
            .filter_map(|row| row.text.map(|text| (row.area, text)))
            .collect())
    }
}
