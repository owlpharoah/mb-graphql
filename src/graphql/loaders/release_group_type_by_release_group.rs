use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct ReleaseGroupTypeRow {
    release_group: i32,
    #[sqlx(rename = "type")]
    type_name: String,
}

pub struct ReleaseGroupTypeLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ReleaseGroupTypeLoader {
    type Value = Option<String>;
    type Error = async_graphql::Error;

    async fn load(&self, release_group_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = release_group_ids.len(), "ReleaseGroupTypeLoader batch load");

        let rows = sqlx::query_as!(
            ReleaseGroupTypeRow,
            r#"
            SELECT rg.id AS release_group, rgt.name AS type_name
            FROM release_group rg
            JOIN release_group_primary_type rgt ON rgt.id = rg.type
            WHERE rg.id = ANY($1)
            "#,
            release_group_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "ReleaseGroupTypeLoader query returned");

        let mut result: HashMap<i32, Option<String>> = HashMap::new();
        for row in rows {
            result.insert(row.release_group, Some(row.type_name));
        }
        for id in release_group_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
