use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct TagNameRow {
    id: i32,
    name: String,
}

pub struct TagLoader {
    pub pool: PgPool,
}

impl Loader<i32> for TagLoader {
    type Value = String;
    type Error = async_graphql::Error;

    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = ids.len(), "TagLoader batch load");

        let rows = sqlx::query_as::<_, TagNameRow>("SELECT id, name FROM tag WHERE id = ANY($1)")
            .bind(ids)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "TagLoader query returned");

        Ok(rows.into_iter().map(|row| (row.id, row.name)).collect())
    }
}
