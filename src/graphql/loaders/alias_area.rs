use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

use crate::graphql::types::common::{Alias, PartialDate};

#[derive(sqlx::FromRow)]
struct AreaAliasRow {
    area: i32,
    name: String,
    sort_name: Option<String>,
    type_name: Option<String>,
    locale: Option<String>,
    primary_for_locale: Option<bool>,
    begin_date_year: Option<i16>,
    begin_date_month: Option<i16>,
    begin_date_day: Option<i16>,
    end_date_year: Option<i16>,
    end_date_month: Option<i16>,
    end_date_day: Option<i16>,
    ended: bool,
}

pub struct AreaAliasLoader {
    pub pool: PgPool,
}

impl Loader<i32> for AreaAliasLoader {
    type Value = Vec<Alias>;
    type Error = async_graphql::Error;

    async fn load(&self, area_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = area_ids.len(), "AreaAliasLoader batch load");

        let rows = sqlx::query_as::<_, AreaAliasRow>(
            r#"SELECT
                    aa.area`,
                    aa.name,
                    aa.sort_name,
                    at.name AS type_name,
                    aa.locale,
                    aa.primary_for_locale,
                    aa.begin_date_year,
                    aa.begin_date_month,
                    aa.begin_date_day,
                    aa.end_date_year,
                    aa.end_date_month,
                    aa.end_date_day,
                    aa.ended
                FROM area_alias aa
                LEFT JOIN area_alias_type at ON at.id = aa."type"
                WHERE aa.area = ANY($1)"#,
        )
        .bind(area_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "AreaAliasLoader query returned");

        let mut result: HashMap<i32, Vec<Alias>> = HashMap::new();
        for row in rows {
            result.entry(row.area).or_default().push(Alias {
                name: row.name,
                sort_name: row.sort_name,
                alias_type: row.type_name,
                locale: row.locale,
                primary: row.primary_for_locale,
                begin_date: PartialDate::from_parts(
                    row.begin_date_year,
                    row.begin_date_month,
                    row.begin_date_day,
                ),
                end_date: PartialDate::from_parts(
                    row.end_date_year,
                    row.end_date_month,
                    row.end_date_day,
                ),
                ended: row.ended,
            });
        }
        for id in area_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
