use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;

#[derive(sqlx::FromRow)]
struct ArtistReleaseIdRow {
    artist: i32,
    release: i32,
}

pub struct ReleaseIdsByArtistLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ReleaseIdsByArtistLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, artist_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let rows = sqlx::query_as::<_, ArtistReleaseIdRow>(
            "SELECT artist , release FROM artist_release WHERE artist = ANY($1)",
        )
        .bind(artist_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let mut result: HashMap<i32, Vec<i32>> = HashMap::new();
        for row in rows {
            result.entry(row.artist).or_default().push(row.release);
        }
        for id in artist_ids {
            result.entry(*id).or_default();
        }
        Ok(result)
    }
}
