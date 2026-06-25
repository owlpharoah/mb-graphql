use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;
#[derive(sqlx::FromRow)]
struct ArtistAreaIdRow {
    artist: i32,
    area: Option<i32>,
}

pub struct AreaIdsByArtistLoader {
    pub pool: PgPool,
}
pub struct BeginAreaIdsByArtistLoader {
    pub pool: PgPool,
}
pub struct EndAreaIdsByArtistLoader {
    pub pool: PgPool,
}

async fn load_area_ids(
    artist_ids: &[i32],
    column: &'static str,
    pool: &PgPool,
) -> Result<HashMap<i32, i32>, async_graphql::Error> {
    info!(count = artist_ids.len(), "AreaIdsByArtistLoader batch load");
    let query = format!(
        "SELECT id AS artist, {} AS area FROM artist WHERE id = ANY($1)",
        column
    );
    let rows = sqlx::query_as::<_, ArtistAreaIdRow>(&query)
        .bind(artist_ids)
        .fetch_all(pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
    info!(rows = rows.len(), "AreaIdsByArtistLoader query returned");
    let mut result = HashMap::new();

    for row in rows {
        if let Some(area) = row.area {
            result.insert(row.artist, area);
        }
    }

    Ok(result)
}
impl Loader<i32> for AreaIdsByArtistLoader {
    type Value = i32;
    type Error = async_graphql::Error;

    async fn load(&self, artist_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        load_area_ids(artist_ids, "area", &self.pool).await
    }
}

impl Loader<i32> for BeginAreaIdsByArtistLoader {
    type Value = i32;
    type Error = async_graphql::Error;

    async fn load(&self, artist_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        load_area_ids(artist_ids, "begin_area", &self.pool).await
    }
}

impl Loader<i32> for EndAreaIdsByArtistLoader {
    type Value = i32;
    type Error = async_graphql::Error;

    async fn load(&self, artist_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        load_area_ids(artist_ids, "end_area", &self.pool).await
    }
}
