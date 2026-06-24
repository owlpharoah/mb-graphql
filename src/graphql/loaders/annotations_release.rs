use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct ReleaseAnnotationRow {
    release: i32,
    text: Option<String>,
}

pub struct ReleaseAnnotationLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ReleaseAnnotationLoader {
    type Value = String;
    type Error = async_graphql::Error;

    async fn load(&self, release_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = release_ids.len(),
            "ReleaseAnnotationLoader batch load"
        );

        let rows = sqlx::query_as::<_, ReleaseAnnotationRow>(
            r#"SELECT ra.release AS release, a.text AS text
            FROM release_annotation ra
            JOIN annotation a ON a.id = ra.annotation
            WHERE ra.release = ANY($1)"#,
        )
        .bind(release_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "ReleaseAnnotationLoader query returned");

        Ok(rows
            .into_iter()
            .filter_map(|row| row.text.map(|text| (row.release, text)))
            .collect())
    }
}
