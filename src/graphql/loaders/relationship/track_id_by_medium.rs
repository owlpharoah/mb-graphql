use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;
#[derive(sqlx::FromRow)]
struct MediumTrackIdRow {
    id: i32,
    medium: i32,
}

pub struct TrackIdByMediumLoader {
    pub pool: PgPool,
}

impl Loader<i32> for TrackIdByMediumLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, m_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = m_ids.len(), "TrackIdByMediumLoader batch load");
        let rows = sqlx::query_as!(
            MediumTrackIdRow,
            "SELECT id , medium FROM track WHERE medium = ANY($1)",
            m_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(rows = rows.len(), "TrackIdByMediumLoader query returned");
        let mut result: HashMap<i32, Vec<i32>> = HashMap::new();
        for row in rows {
            result.entry(row.medium).or_default().push(row.id);
        }
        for id in m_ids {
            result.entry(*id).or_default();
        }
        Ok(result)
    }
}
