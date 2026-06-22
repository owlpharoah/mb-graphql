use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;
#[derive(sqlx::FromRow)]
struct ReleaseGroupReleaseIdRow {
    id: i32,
    release_group: i32,
}

pub struct ReleaseIdByReleaseGroupLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ReleaseIdByReleaseGroupLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, rg_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = rg_ids.len(),
            "ReleaseIdByReleaseGroupLoader batch load"
        );
        let rows = sqlx::query_as::<_, ReleaseGroupReleaseIdRow>(
            "SELECT id , release_group FROM release WHERE release_group = ANY($1)",
        )
        .bind(rg_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(
            rows = rows.len(),
            "ReleaseIdByReleaseGroupLoader query returned"
        );
        let mut result: HashMap<i32, Vec<i32>> = HashMap::new();
        for row in rows {
            result.entry(row.release_group).or_default().push(row.id);
        }
        for id in rg_ids {
            result.entry(*id).or_default();
        }
        Ok(result)
    }
}
