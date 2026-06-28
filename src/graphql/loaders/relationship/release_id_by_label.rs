use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;
#[derive(sqlx::FromRow)]
struct LabelReleaseIdRow {
    label: i32,
    release: i32,
}

pub struct ReleaseIdsByLabelLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ReleaseIdsByLabelLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, label_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = label_ids.len(),
            "ReleaseIdsByLabelLoader batch load"
        );
        let rows = sqlx::query_as!(
            LabelReleaseIdRow,
            r#"
                SELECT DISTINCT
                    label AS "label!",
                    release
                FROM release_label
                WHERE label = ANY($1)
                "#,
            label_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(rows = rows.len(), "ReleaseIdsByLabelLoader query returned");
        let mut result: HashMap<i32, Vec<i32>> = HashMap::new();
        for row in rows {
            result.entry(row.label).or_default().push(row.release);
        }
        for id in label_ids {
            result.entry(*id).or_default();
        }
        Ok(result)
    }
}
