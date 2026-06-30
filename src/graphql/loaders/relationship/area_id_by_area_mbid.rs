use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct AreaIDMBIDRow {
    gid: Uuid,
    id: i32,
}
#[derive(sqlx::FromRow)]
struct AreaRedirectRow {
    gid: Uuid,
    new_id: i32,
}

pub struct AreaIDByMBIDLoader {
    pub pool: PgPool,
}

impl Loader<Uuid> for AreaIDByMBIDLoader {
    type Value = i32;
    type Error = async_graphql::Error;

    async fn load(&self, area_mbids: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        info!(count = area_mbids.len(), "AreaIDByMBIDLoader batch load");
        let rows = sqlx::query_as!(
            AreaIDMBIDRow,
            "SELECT gid, id FROM area WHERE gid = ANY($1)",
            area_mbids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(rows = rows.len(), "AreaIDByMBIDLoader query returned");
        let mut result: HashMap<Uuid, i32> = HashMap::new();
        for row in rows {
            result.insert(row.gid, row.id);
        }
        let unresolved: Vec<Uuid> = area_mbids
            .iter()
            .filter(|gid| !result.contains_key(gid))
            .copied()
            .collect();

        if !unresolved.is_empty() {
            let redirects = sqlx::query_as!(
                AreaRedirectRow,
                "SELECT gid, new_id FROM area_gid_redirect WHERE gid = ANY($1)",
                &unresolved
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

            info!(
                redirects = redirects.len(),
                "AreaIDByMBIDLoader redirect lookup returned"
            );

            for row in redirects {
                result.insert(row.gid, row.new_id);
            }
        }
        Ok(result)
    }
}
