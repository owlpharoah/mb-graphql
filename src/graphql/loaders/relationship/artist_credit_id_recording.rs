use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct RecordingArtistCreditRow {
    id: i32,
    artist_credit: i32,
}

pub struct ArtistCreditIdByRecordingLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ArtistCreditIdByRecordingLoader {
    type Value = i32;
    type Error = async_graphql::Error;

    async fn load(&self, recording_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = recording_ids.len(),
            "ArtistCreditIdByRecordingLoader batch load"
        );

        let rows = sqlx::query_as!(
            RecordingArtistCreditRow,
            "SELECT id, artist_credit FROM recording WHERE id = ANY($1)",
            recording_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(
            rows = rows.len(),
            "ArtistCreditIdByRecordingLoader query returned"
        );

        Ok(rows
            .into_iter()
            .map(|row| (row.id, row.artist_credit))
            .collect())
    }
}
