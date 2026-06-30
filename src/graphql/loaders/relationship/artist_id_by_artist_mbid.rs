use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;
use uuid::Uuid;
#[derive(sqlx::FromRow)]
struct ArtistIDMBIDRow {
    gid: Uuid,
    id: i32,
}

#[derive(sqlx::FromRow)]
struct ArtistRedirectRow {
    gid: Uuid,
    new_id: i32,
}

pub struct ArtistIDByMBIDLoader {
    pub pool: PgPool,
}

impl Loader<Uuid> for ArtistIDByMBIDLoader {
    type Value = i32;
    type Error = async_graphql::Error;

    async fn load(&self, artist_mbids: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        info!(
            count = artist_mbids.len(),
            "ArtistIDByMBIDLoader batch load"
        );
        let rows = sqlx::query_as!(
            ArtistIDMBIDRow,
            "SELECT gid , id FROM artist WHERE gid = ANY($1)",
            artist_mbids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(rows = rows.len(), "ArtistIDByMBIDLoader query returned");
        let mut result: HashMap<Uuid, i32> = HashMap::new();
        for row in rows {
            result.insert(row.gid, row.id);
        }

        let unresolved: Vec<Uuid> = artist_mbids
            .iter()
            .filter(|gid| !result.contains_key(gid))
            .copied()
            .collect();

        if !unresolved.is_empty() {
            let redirects = sqlx::query_as!(
                ArtistRedirectRow,
                "SELECT gid, new_id FROM artist_gid_redirect WHERE gid = ANY($1)",
                &unresolved
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

            info!(
                redirects = redirects.len(),
                "ArtistIDByMBIDLoader redirect lookup returned"
            );

            for row in redirects {
                result.insert(row.gid, row.new_id);
            }
        }

        Ok(result)
    }
}
