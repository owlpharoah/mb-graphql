use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;
use uuid::Uuid;

use crate::graphql::types::common::Genre;

#[derive(sqlx::FromRow)]
struct GenreRow {
    id: i32,
    gid: Uuid,
    name: String,
    comment: Option<String>,
}

pub struct GenreLoader {
    pub pool: PgPool,
}

impl Loader<i32> for GenreLoader {
    type Value = Genre;
    type Error = async_graphql::Error;

    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = ids.len(), "GenreLoader batch load");

        let rows = sqlx::query_as::<_, GenreRow>(
            "SELECT id, gid, name, comment FROM genre WHERE id = ANY($1)",
        )
        .bind(ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "GenreLoader query returned");

        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    row.id,
                    Genre {
                        mbid: row.gid,
                        name: row.name,
                        disambiguation: row.comment,
                    },
                )
            })
            .collect())
    }
}
