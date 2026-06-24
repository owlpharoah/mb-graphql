use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct ReleaseGroupAnnotationRow {
    release_group: i32,
    text: Option<String>,
}

pub struct ReleaseGroupAnnotationLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ReleaseGroupAnnotationLoader {
    type Value = String;
    type Error = async_graphql::Error;

    async fn load(
        &self,
        release_group_ids: &[i32],
    ) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = release_group_ids.len(),
            "ReleaseGroupAnnotationLoader batch load"
        );

        let rows = sqlx::query_as::<_, ReleaseGroupAnnotationRow>(
            r#"SELECT rga.release_group AS release_group, a.text AS text
            FROM release_group_annotation rga
            JOIN annotation a ON a.id = rga.annotation
            WHERE rga.release_group = ANY($1)"#,
        )
        .bind(release_group_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(
            rows = rows.len(),
            "ReleaseGroupAnnotationLoader query returned"
        );

        Ok(rows
            .into_iter()
            .filter_map(|row| row.text.map(|text| (row.release_group, text)))
            .collect())
    }
}
