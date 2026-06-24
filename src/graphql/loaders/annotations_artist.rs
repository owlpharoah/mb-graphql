use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct ArtistAnnotationRow {
    artist: i32,
    text: Option<String>,
}

pub struct ArtistAnnotationLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ArtistAnnotationLoader {
    type Value = String;
    type Error = async_graphql::Error;

    async fn load(&self, artist_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = artist_ids.len(),
            "ArtistAnnotationLoader batch load"
        );

        let rows = sqlx::query_as::<_, ArtistAnnotationRow>(
            r#"SELECT aa.artist AS artist, a.text AS text
            FROM artist_annotation aa
            JOIN annotation a ON a.id = aa.annotation
            WHERE aa.artist = ANY($1)"#,
        )
        .bind(artist_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "ArtistAnnotationLoader query returned");

        Ok(rows
            .into_iter()
            .filter_map(|row| row.text.map(|text| (row.artist, text)))
            .collect())
    }
}
