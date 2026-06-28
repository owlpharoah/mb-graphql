use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct ArtistGenreIdRow {
    artist: i32,
    id: i32,
}

pub struct GenreIdsByArtistLoader {
    pub pool: PgPool,
}

impl Loader<i32> for GenreIdsByArtistLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, artist_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = artist_ids.len(),
            "GenreIdsByArtistLoader batch load"
        );

        let rows = sqlx::query_as!(
            ArtistGenreIdRow,
            r#"SELECT at.artist AS artist, g.id AS id
            FROM artist_tag at
            JOIN tag t ON t.id = at.tag
            JOIN genre g ON g.name = t.name
            WHERE at.artist = ANY($1)
            ORDER BY at.artist, at.count DESC"#,
            artist_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "GenreIdsByArtistLoader query returned");

        let mut result: HashMap<i32, Vec<i32>> = HashMap::new();
        for row in rows {
            result.entry(row.artist).or_default().push(row.id);
        }
        for id in artist_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
