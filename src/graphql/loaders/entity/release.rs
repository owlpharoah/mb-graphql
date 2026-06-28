use crate::graphql::types::release::{Release, ReleaseRow};
use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

pub struct ReleaseLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ReleaseLoader {
    type Value = Release;
    type Error = async_graphql::Error;

    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = ids.len(), "ReleaseLoader batch load");
        let rows = sqlx::query_as!(
            ReleaseRow,
            r#"SELECT
                id,
                gid,
                name,
                release_group,
                status,
                packaging,
                quality,
                language,
                script,
                barcode,
                comment
            FROM release WHERE id = ANY($1)"#,
            ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(rows = rows.len(), "ReleaseLoader query returned");
        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    row.id,
                    Release {
                        mbid: row.gid,
                        name: row.name,
                        disambiguation: row.comment,
                        status: row.status,
                        packaging: row.packaging,
                        quality: row.quality,
                        barcode: row.barcode,
                        language: row.language,
                        script: row.script,
                        release_group: row.release_group,
                        id: row.id,
                    },
                )
            })
            .collect())
    }
}
