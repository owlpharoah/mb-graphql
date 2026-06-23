use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

use crate::graphql::types::common::ArtistCredit;

#[derive(sqlx::FromRow)]
struct ArtistCreditNameRow {
    artist_credit: i32,
    artist: i32,
    name: String,
    join_phrase: String,
}

pub struct ArtistCreditLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ArtistCreditLoader {
    type Value = Vec<ArtistCredit>;
    type Error = async_graphql::Error;

    async fn load(&self, credit_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = credit_ids.len(), "ArtistCreditLoader batch load");

        let rows = sqlx::query_as::<_, ArtistCreditNameRow>(
            r#"SELECT
                artist_credit,
                artist,
                name,
                join_phrase
            FROM artist_credit_name
            WHERE artist_credit = ANY($1)
            ORDER BY artist_credit, position"#,
        )
        .bind(credit_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "ArtistCreditLoader query returned");

        let mut result: HashMap<i32, Vec<ArtistCredit>> = HashMap::new();
        for row in rows {
            result
                .entry(row.artist_credit)
                .or_default()
                .push(ArtistCredit {
                    name: row.name,
                    join_phrase: row.join_phrase,
                    artist_id: row.artist,
                });
        }
        for id in credit_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
