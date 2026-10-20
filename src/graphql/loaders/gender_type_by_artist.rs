use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct GenderTypeRow {
    artist: i32,
    #[sqlx(rename = "type")]
    type_name: String,
}

pub struct GenderTypeLoader {
    pub pool: PgPool,
}

impl Loader<i32> for GenderTypeLoader {
    type Value = Option<String>;
    type Error = async_graphql::Error;

    async fn load(&self, artist_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = artist_ids.len(), "GenderTypeLoader batch load");

        let rows = sqlx::query_as!(
            GenderTypeRow,
            r#"
            SELECT a.id AS artist, g.name AS type_name
            FROM artist a
            JOIN gender g ON g.id = a.type
            WHERE a.id = ANY($1)
            "#,
            artist_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "GenderTypeLoader query returned");

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
