use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct ReleasePackagingRow {
    release: i32,
    #[sqlx(rename = "type")]
    type_name: String,
}

pub struct ReleasePackagingLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ReleasePackagingLoader {
    type Value = Option<String>;
    type Error = async_graphql::Error;

    async fn load(&self, release_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = release_ids.len(),
            "ReleasePackagingLoader batch load"
        );

        let rows = sqlx::query_as!(
            ReleasePackagingRow,
            r#"
            SELECT r.id AS release, rp.name AS type_name
            FROM release r
            JOIN release_packaging rp ON rp.id = r.packaging
            WHERE r.id = ANY($1)
            "#,
            release_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "ReleasePackagingLoader query returned");

        let mut result: HashMap<i32, Option<String>> = HashMap::new();
        for row in rows {
            result.insert(row.release, Some(row.type_name));
        }
        for id in release_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
