use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct LabelAreaRow {
    id: i32,
    area: Option<i32>,
}

pub struct AreaIdByLabelLoader {
    pub pool: PgPool,
}

impl Loader<i32> for AreaIdByLabelLoader {
    type Value = Option<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, label_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = label_ids.len(), "AreaIdByLabelLoader batch load");

        let rows =
            sqlx::query_as::<_, LabelAreaRow>("SELECT id, area FROM label WHERE id = ANY($1)")
                .bind(label_ids)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "AreaIdByLabelLoader query returned");

        Ok(rows.into_iter().map(|row| (row.id, row.area)).collect())
    }
}
