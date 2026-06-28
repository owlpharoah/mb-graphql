use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct ReleaseArtistCreditRow {
    id: i32,
    artist_credit: i32,
}

pub struct ArtistCreditIdByReleaseLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ArtistCreditIdByReleaseLoader {
    type Value = i32;
    type Error = async_graphql::Error;

    async fn load(&self, release_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = release_ids.len(),
            "ArtistCreditIdByReleaseLoader batch load"
        );

        let rows = sqlx::query_as!(
            ReleaseArtistCreditRow,
            "SELECT id, artist_credit FROM release WHERE id = ANY($1)",
            release_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(
            rows = rows.len(),
            "ArtistCreditIdByReleaseLoader query returned"
        );

        Ok(rows
            .into_iter()
            .map(|row| (row.id, row.artist_credit))
            .collect())
    }
}
