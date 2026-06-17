use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;

use crate::graphql::types::release::{Release, ReleaseRow};

pub struct ReleaseByGroupLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ReleaseByGroupLoader {
    type Value = Vec<Release>;
    type Error = async_graphql::Error;

    async fn load(&self, rg_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let rows = sqlx::query_as::<_, ReleaseRow>(
            r#"
            SELECT
                r.id,
                r.gid,
                r.name,
                r.artist_credit,
                r.release_group,
                r.status,
                r.packaging,
                r.quality,
                r.language,
                r.script,
                r.barcode,
                r.comment
            FROM release r WHERE r.release_group = ANY($1)
                    "#,
        )
        .bind(rg_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let mut result: HashMap<i32, Vec<Release>> = HashMap::new();

        for row in rows {
            result.entry(row.release_group).or_default().push(Release {
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
            });
        }
        for rg_id in rg_ids {
            result.entry(*rg_id).or_default();
        }

        Ok(result)
    }
}
