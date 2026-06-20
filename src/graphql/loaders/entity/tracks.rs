use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use crate::graphql::types::common::Track;

pub struct TrackLoader {
    pub pool: PgPool,
}

#[derive(sqlx::FromRow)]
pub struct TrackRow {
    id: i32,
    gid: Uuid,
    recording: i32,
    medium: i32,
    position: i32,
    number: String,
    artist_credit: i32,
    length: Option<i32>,
    name: String,
}

impl Loader<i32> for TrackLoader {
    type Value = Track;
    type Error = async_graphql::Error;

    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let rows = sqlx::query_as::<_, TrackRow>(
            r#"SELECT
            id,
            gid,
            recording,
            medium,
            number,
            position,
            artist_credit,
            length,
            name
            FROM track
            WHERE id = ANY($1)"#,
        )
        .bind(ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    row.id,
                    Track {
                        mbid: row.gid,
                        name: row.name,
                        number: row.number,
                        position: row.position,
                        length: row.length,
                        artist_credit: row.artist_credit,
                        recording_id: row.recording,
                        medium: row.medium,
                        id: row.id,
                    },
                )
            })
            .collect())
    }
}
