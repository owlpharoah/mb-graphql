use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;

use crate::graphql::types::release::{Release, ReleaseRow};

pub struct ReleaseLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ReleaseLoader {
    type Value = Release;
    type Error = async_graphql::Error;

    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let rows = sqlx::query_as::<_, ReleaseRow>(
            r#"SELECT
                id,
                gid,
                name,
                artist_credit,
                release_group,
                status,
                packaging,
                quality,
                language,
                script,
                barcode,
                comment
            FROM release WHERE id = ANY($1)"#,
        )
        .bind(ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

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
                        artist_credit: row.artist_credit,
                    },
                )
            })
            .collect())
    }
}
