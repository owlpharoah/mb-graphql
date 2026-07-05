use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct ArtistReleaseGroupIdRow {
    artist: i32,
    release_group: i32,
}

pub struct ReleaseGroupIdsByArtistLoader {
    pub pool: PgPool,
}
use crate::graphql::loaders::relationship::PageKey;
impl Loader<PageKey> for ReleaseGroupIdsByArtistLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, keys: &[PageKey]) -> Result<HashMap<PageKey, Self::Value>, Self::Error> {
        info!(
            count = keys.len(),
            "ReleaseGroupIdsByArtistLoader batch load"
        );

        let mut groups: HashMap<(Option<i32>, i32), Vec<i32>> = HashMap::new();
        for key in keys {
            groups
                .entry((key.after, key.first))
                .or_default()
                .push(key.entity_id);
        }

        let mut result: HashMap<PageKey, Vec<i32>> = HashMap::new();

        for ((after, first), entity_ids) in groups {
            let rows = sqlx::query_as!(
                ArtistReleaseGroupIdRow,
                r#"SELECT artist AS "artist!", release_group AS "release_group!"
                FROM (
                    SELECT artist, release_group,
                           ROW_NUMBER() OVER (PARTITION BY artist ORDER BY release_group) AS rn
                    FROM artist_release_group
                    WHERE artist = ANY($1)
                      AND ($2::int IS NULL OR release_group > $2)
                ) ranked
                WHERE rn <= $3
                ORDER BY artist, release_group"#,
                &entity_ids,
                after,
                first as i64
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

            let mut group_result: HashMap<i32, Vec<i32>> = HashMap::new();
            for row in rows {
                group_result
                    .entry(row.artist)
                    .or_default()
                    .push(row.release_group);
            }

            for id in &entity_ids {
                result.insert(
                    PageKey {
                        entity_id: *id,
                        after,
                        first,
                    },
                    group_result.remove(id).unwrap_or_default(),
                );
            }
        }

        Ok(result)
    }
}
