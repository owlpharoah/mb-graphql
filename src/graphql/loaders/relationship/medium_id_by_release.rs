use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;
#[derive(sqlx::FromRow)]
struct ReleaseMediumIdRow {
    id: i32,
    release: i32,
}

pub struct MediumIdByReleaseLoader {
    pub pool: PgPool,
}

impl Loader<i32> for MediumIdByReleaseLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, r_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = r_ids.len(), "MediumIdByReleaseLoader batch load");
        let rows = sqlx::query_as::<_, ReleaseMediumIdRow>(
            "SELECT id , release FROM medium WHERE release = ANY($1)",
        )
        .bind(r_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(rows = rows.len(), "MediumIdByReleaseLoader query returned");
        let mut result: HashMap<i32, Vec<i32>> = HashMap::new();
        for row in rows {
            result.entry(row.release).or_default().push(row.id);
        }
        for id in r_ids {
            result.entry(*id).or_default();
        }
        Ok(result)
    }
}
