use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;

#[derive(sqlx::FromRow)]
pub struct AreaISOCodeRow {
    area: i32,
    code: String,
}

pub struct IsoCode2ByAreaLoader {
    pub pool: PgPool,
}

impl Loader<i32> for IsoCode2ByAreaLoader {
    type Value = Vec<String>;
    type Error = async_graphql::Error;

    async fn load(&self, area_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let rows = sqlx::query_as::<_, AreaISOCodeRow>(
            "SELECT
                area,code
            FROM iso_3166_2
            WHERE area = ANY($1)",
        )
        .bind(area_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        let mut result: HashMap<i32, Vec<String>> = HashMap::new();
        for row in rows {
            result.entry(row.area).or_default().push(row.code);
        }
        for id in area_ids {
            result.entry(*id).or_default();
        }
        Ok(result)
    }
}
