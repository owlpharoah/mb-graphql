use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

use crate::graphql::types::common::{Alias, PartialDate};

#[derive(sqlx::FromRow)]
struct LabelAliasRow {
    label: i32,
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

pub struct LabelAliasLoader {
    pub pool: PgPool,
}

impl Loader<i32> for LabelAliasLoader {
    type Value = Vec<Alias>;
    type Error = async_graphql::Error;

    async fn load(&self, label_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = label_ids.len(), "LabelAliasLoader batch load");

        let rows = sqlx::query_as!(
            LabelAliasRow,
            r#"SELECT
                la.label,
                la.name,
                la.sort_name,
                at.name AS type_name,
                la.locale,
                la.primary_for_locale,
                la.begin_date_year,
                la.begin_date_month,
                la.begin_date_day,
                la.end_date_year,
                la.end_date_month,
                la.end_date_day,
                la.ended
            FROM label_alias la
            LEFT JOIN label_alias_type at ON at.id = la."type"
            WHERE la.label = ANY($1)"#,
            label_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "LabelAliasLoader query returned");

        let mut result: HashMap<i32, Vec<Alias>> = HashMap::new();
        for row in rows {
            result.entry(row.label).or_default().push(Alias {
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
        for id in label_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
