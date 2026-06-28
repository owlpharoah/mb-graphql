use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

use crate::graphql::types::common::Rating;

#[derive(sqlx::FromRow)]
struct ArtistRatingRow {
    id: i32,
    rating: Option<i16>,
    rating_count: Option<i32>,
}

pub struct ArtistRatingLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ArtistRatingLoader {
    type Value = Rating;
    type Error = async_graphql::Error;

    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = ids.len(), "ArtistRatingLoader batch load");

        let rows = sqlx::query_as!(
            ArtistRatingRow,
            "SELECT id, rating, rating_count FROM artist_meta WHERE id = ANY($1)",
            ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "ArtistRatingLoader query returned");

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
