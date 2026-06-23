use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

use crate::graphql::types::common::Rating;

#[derive(sqlx::FromRow)]
struct RecordingRatingRow {
    id: i32,
    rating: Option<i16>,
    rating_count: Option<i32>,
}

pub struct RecordingRatingLoader {
    pub pool: PgPool,
}

impl Loader<i32> for RecordingRatingLoader {
    type Value = Rating;
    type Error = async_graphql::Error;

    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = ids.len(), "RecordingRatingLoader batch load");

        let rows = sqlx::query_as::<_, RecordingRatingRow>(
            "SELECT id, rating, rating_count FROM recording_meta WHERE id = ANY($1)",
        )
        .bind(ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "RecordingRatingLoader query returned");

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
