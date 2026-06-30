use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct LabelIDMBIDRow {
    gid: Uuid,
    id: i32,
}
#[derive(sqlx::FromRow)]
struct LabelRedirectRow {
    gid: Uuid,
    new_id: i32,
}

pub struct LabelIDByMBIDLoader {
    pub pool: PgPool,
}

impl Loader<Uuid> for LabelIDByMBIDLoader {
    type Value = i32;
    type Error = async_graphql::Error;

    async fn load(&self, label_mbids: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        info!(count = label_mbids.len(), "LabelIDByMBIDLoader batch load");
        let rows = sqlx::query_as!(
            LabelIDMBIDRow,
            "SELECT gid, id FROM label WHERE gid = ANY($1)",
            label_mbids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(rows = rows.len(), "LabelIDByMBIDLoader query returned");
        let mut result: HashMap<Uuid, i32> = HashMap::new();
        for row in rows {
            result.insert(row.gid, row.id);
        }
        let unresolved: Vec<Uuid> = label_mbids
            .iter()
            .filter(|gid| !result.contains_key(gid))
            .copied()
            .collect();

        if !unresolved.is_empty() {
            let redirects = sqlx::query_as!(
                LabelRedirectRow,
                "SELECT gid, new_id FROM label_gid_redirect WHERE gid = ANY($1)",
                &unresolved
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

            info!(
                redirects = redirects.len(),
                "LabelIDByMBIDLoader redirect lookup returned"
            );

            for row in redirects {
                result.insert(row.gid, row.new_id);
            }
        }
        Ok(result)
    }
}
