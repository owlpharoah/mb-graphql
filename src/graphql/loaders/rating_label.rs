use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

use crate::graphql::types::common::Rating;

#[derive(sqlx::FromRow)]
struct LabelRatingRow {
    id: i32,
    rating: Option<i16>,
    rating_count: Option<i32>,
}

pub struct LabelRatingLoader {
    pub pool: PgPool,
}

impl Loader<i32> for LabelRatingLoader {
    type Value = Rating;
    type Error = async_graphql::Error;

    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = ids.len(), "LabelRatingLoader batch load");

        let rows = sqlx::query_as!(
            LabelRatingRow,
            "SELECT id, rating, rating_count FROM label_meta WHERE id = ANY($1)",
            ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "LabelRatingLoader query returned");

        Ok(rows
            .into_iter()
            .filter_map(|row| {
                row.rating.map(|value| {
                    (
                        row.id,
                        Rating {
                            value,
                            votes_count: row.rating_count,
                        },
                    )
                })
            })
            .collect())
    }
}
