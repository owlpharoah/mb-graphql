use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct ArtistTypeRow {
    artist: i32,
    #[sqlx(rename = "type")]
    type_name: String,
}

pub struct ArtistTypeLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ArtistTypeLoader {
    type Value = Option<String>;
    type Error = async_graphql::Error;

    async fn load(&self, artist_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = artist_ids.len(), "ArtistTypeLoader batch load");

        let rows = sqlx::query_as!(
            ArtistTypeRow,
            r#"
            SELECT a.id AS artist, at.name AS type_name
            FROM area a
            JOIN artist_type at ON at.id = a.type
            WHERE a.id = ANY($1)
            "#,
            artist_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "ArtistTypeLoader query returned");

        let mut result: HashMap<i32, Option<String>> = HashMap::new();
        for row in rows {
            result.insert(row.artist, Some(row.type_name));
        }
        for id in artist_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
