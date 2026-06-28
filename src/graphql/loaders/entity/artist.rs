use crate::graphql::types::{
    artist::{Artist, ArtistRow},
    common::PartialDate,
};
use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

pub struct ArtistLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ArtistLoader {
    type Value = Artist;
    type Error = async_graphql::Error;

    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = ids.len(), "ArtistLoader batch load");
        let rows = sqlx::query_as!(
            ArtistRow,
            r#"SELECT id, gid, name, sort_name, comment,type AS artist_type, gender, ended, begin_date_year, begin_date_month, begin_date_day, end_date_year, end_date_month, end_date_day FROM artist WHERE id = ANY($1)"#,
            ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(rows = rows.len(), "ArtistLoader query returned");
        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    row.id,
                    Artist {
                        mbid: row.gid,
                        name: row.name,
                        sort_name: row.sort_name,
                        disambiguation: row.comment,
                        artist_type: row.artist_type,
                        gender: row.gender,
                        ended: row.ended,
                        begin_date: PartialDate::from_parts(
                            row.begin_date_year,
                            row.begin_date_month,
                            row.begin_date_day,
                        ),
                        end_date: PartialDate::from_parts(
                            row.end_date_year,
                            row.end_date_month,
                            row.end_date_day,
                        ),
                        id: row.id,
                    },
                )
            })
            .collect())
    }
}
