use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct ReleaseGroupArtistCreditRow {
    id: i32,
    artist_credit: i32,
}

pub struct ArtistCreditIdByReleaseGroupLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ArtistCreditIdByReleaseGroupLoader {
    type Value = i32;
    type Error = async_graphql::Error;

    async fn load(
        &self,
        release_group_ids: &[i32],
    ) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = release_group_ids.len(),
            "ArtistCreditIdByReleaseGroupLoader batch load"
        );

        let rows = sqlx::query_as!(
            ReleaseGroupArtistCreditRow,
            "SELECT id, artist_credit FROM release_group WHERE id = ANY($1)",
            release_group_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(
            rows = rows.len(),
            "ArtistCreditIdByReleaseGroupLoader query returned"
        );

        Ok(rows
            .into_iter()
            .map(|row| (row.id, row.artist_credit))
            .collect())
    }
}
