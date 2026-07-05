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
            r#"SELECT gid AS "gid!", id AS "id!" FROM (
                SELECT gid, id FROM label WHERE gid = ANY($1)
                UNION ALL
                SELECT gid, new_id AS id FROM label_gid_redirect WHERE gid = ANY($1)
            ) combined"#,
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
        Ok(result)
    }
}
