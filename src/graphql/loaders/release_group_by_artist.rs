use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use crate::graphql::types::release_group::ReleaseGroup;

#[derive(sqlx::FromRow)]
struct ArtistReleaseGroupRow {
    gid: Uuid,
    name: String,
    artist: i32,
    release_group: i32,
    artist_credit: i32,
    comment: Option<String>,
    primary_type: Option<i16>,
}

pub struct ReleaseGroupByArtistLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ReleaseGroupByArtistLoader {
    type Value = Vec<ReleaseGroup>;
    type Error = async_graphql::Error;

    async fn load(&self, artist_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let rows = sqlx::query_as::<_, ArtistReleaseGroupRow>(
            r#"
                    SELECT
                        arg.artist,
                        arg.release_group,
                        arg.primary_type,
                        rg.name,
                        rg.artist_credit,
                        rg.comment,
                        rg.gid
                    FROM artist_release_group arg
                    JOIN release_group rg ON rg.id = arg.release_group
                    WHERE arg.artist = ANY($1)
                    "#,
        )
        .bind(artist_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let mut result: HashMap<i32, Vec<ReleaseGroup>> = HashMap::new();

        for row in rows {
            result.entry(row.artist).or_default().push(ReleaseGroup {
                mbid: row.gid,
                name: row.name,
                disambiguation: row.comment,
                release_group_type: row.primary_type.map(|x| x as i32),
                id: row.release_group,
                artist_credit: row.artist_credit,
            });
        }
        for artist_id in artist_ids {
            result.entry(*artist_id).or_default();
        }

        Ok(result)
    }
}
