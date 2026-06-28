use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

use crate::graphql::types::common::{Alias, PartialDate};

#[derive(sqlx::FromRow)]
struct ReleaseAliasRow {
    release: i32,
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

pub struct ReleaseAliasLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ReleaseAliasLoader {
    type Value = Vec<Alias>;
    type Error = async_graphql::Error;

    async fn load(&self, release_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = release_ids.len(), "ReleaseAliasLoader batch load");

        let rows = sqlx::query_as!(
            ReleaseAliasRow,
            r#"SELECT
                ra.release,
                ra.name,
                ra.sort_name,
                at.name AS type_name,
                ra.locale,
                ra.primary_for_locale,
                ra.begin_date_year,
                ra.begin_date_month,
                ra.begin_date_day,
                ra.end_date_year,
                ra.end_date_month,
                ra.end_date_day,
                ra.ended
            FROM release_alias ra
            LEFT JOIN release_alias_type at ON at.id = ra."type"
            WHERE ra.release = ANY($1)"#,
            release_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "ReleaseAliasLoader query returned");

        let mut result: HashMap<i32, Vec<Alias>> = HashMap::new();
        for row in rows {
            result.entry(row.release).or_default().push(Alias {
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
        for id in release_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
