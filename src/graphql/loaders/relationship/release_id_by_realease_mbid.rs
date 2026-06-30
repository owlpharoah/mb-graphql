use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct ReleaseIDMBIDRow {
    gid: Uuid,
    id: i32,
}
#[derive(sqlx::FromRow)]
struct ReleaseRedirectRow {
    gid: Uuid,
    new_id: i32,
}

pub struct ReleaseIDByMBIDLoader {
    pub pool: PgPool,
}

impl Loader<Uuid> for ReleaseIDByMBIDLoader {
    type Value = i32;
    type Error = async_graphql::Error;

    async fn load(
        &self,
        release_mbids: &[Uuid],
    ) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        info!(
            count = release_mbids.len(),
            "ReleaseIDByMBIDLoader batch load"
        );
        let rows = sqlx::query_as!(
            ReleaseIDMBIDRow,
            "SELECT gid, id FROM release WHERE gid = ANY($1)",
            release_mbids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(rows = rows.len(), "ReleaseIDByMBIDLoader query returned");
        let mut result: HashMap<Uuid, i32> = HashMap::new();
        for row in rows {
            result.insert(row.gid, row.id);
        }
        let unresolved: Vec<Uuid> = release_mbids
            .iter()
            .filter(|gid| !result.contains_key(gid))
            .copied()
            .collect();

        if !unresolved.is_empty() {
            let redirects = sqlx::query_as!(
                ReleaseRedirectRow,
                "SELECT gid, new_id FROM release_gid_redirect WHERE gid = ANY($1)",
                &unresolved
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

            info!(
                redirects = redirects.len(),
                "ReleaseIDByMBIDLoader redirect lookup returned"
            );

            for row in redirects {
                result.insert(row.gid, row.new_id);
            }
        }
        Ok(result)
    }
}
