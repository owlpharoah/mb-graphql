use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct ArtistIpiRow {
    artist: i32,
    ipi: String,
}

pub struct ArtistIpiLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ArtistIpiLoader {
    type Value = Vec<String>;
    type Error = async_graphql::Error;

    async fn load(&self, artist_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = artist_ids.len(), "ArtistIpiLoader batch load");

        let rows = sqlx::query_as::<_, ArtistIpiRow>(
            "SELECT artist, ipi FROM artist_ipi WHERE artist = ANY($1)",
        )
        .bind(artist_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "ArtistIpiLoader query returned");

        let mut result: HashMap<i32, Vec<String>> = HashMap::new();
        for row in rows {
            result.entry(row.artist).or_default().push(row.ipi);
        }
        for id in artist_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
