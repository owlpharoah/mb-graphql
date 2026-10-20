use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct AreaTypeRow {
    area: i32,
    #[sqlx(rename = "type")]
    type_name: String,
}

pub struct AreaTypeLoader {
    pub pool: PgPool,
}

impl Loader<i32> for AreaTypeLoader {
    type Value = Option<String>;
    type Error = async_graphql::Error;

    async fn load(&self, area_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = area_ids.len(), "AreaTypeLoader batch load");

        let rows = sqlx::query_as!(
            AreaTypeRow,
            r#"
            SELECT a.id AS area, at.name AS type_name
            FROM area a
            JOIN area_type at ON at.id = a.type
            WHERE a.id = ANY($1)
            "#,
            area_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "AreaTypeLoader query returned");

        let mut result: HashMap<i32, Option<String>> = HashMap::new();
        for row in rows {
            result.insert(row.area, Some(row.type_name));
        }
        for id in area_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
