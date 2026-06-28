use crate::graphql::types::common::TagRef;
use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct ArtistTagRow {
    artist: i32,
    tag: i32,
    count: i32,
}

pub struct TagIdsByArtistLoader {
    pub pool: PgPool,
}

impl Loader<i32> for TagIdsByArtistLoader {
    type Value = Vec<TagRef>;
    type Error = async_graphql::Error;

    async fn load(&self, artist_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = artist_ids.len(), "TagIdsByArtistLoader batch load");

        let rows = sqlx::query_as!(
            ArtistTagRow,
            "SELECT artist, tag, count FROM artist_tag WHERE artist = ANY($1)",
            artist_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "TagIdsByArtistLoader query returned");

        let mut result: HashMap<i32, Vec<TagRef>> = HashMap::new();
        for row in rows {
            result.entry(row.artist).or_default().push(TagRef {
                tag_id: row.tag,
                count: row.count,
            });
        }
        for id in artist_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
