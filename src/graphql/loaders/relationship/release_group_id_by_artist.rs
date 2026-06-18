use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;

#[derive(sqlx::FromRow)]
struct ArtistReleaseGroupIdRow {
    artist: i32,
    release_group: i32,
}

pub struct ReleaseGroupIdsByArtistLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ReleaseGroupIdsByArtistLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, artist_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let rows = sqlx::query_as::<_, ArtistReleaseGroupIdRow>(
            "SELECT artist, release_group FROM artist_release_group WHERE artist = ANY($1)",
        )
        .bind(artist_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let mut result: HashMap<i32, Vec<i32>> = HashMap::new();
        for row in rows {
            result
                .entry(row.artist)
                .or_default()
                .push(row.release_group);
        }
        for id in artist_ids {
            result.entry(*id).or_default();
        }
        Ok(result)
    }
}
