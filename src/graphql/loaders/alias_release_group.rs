use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

use crate::graphql::types::common::{Alias, PartialDate};

#[derive(sqlx::FromRow)]
struct ReleaseGroupAliasRow {
    release_group: i32,
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

pub struct ReleaseGroupAliasLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ReleaseGroupAliasLoader {
    type Value = Vec<Alias>;
    type Error = async_graphql::Error;

    async fn load(
        &self,
        release_group_ids: &[i32],
    ) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = release_group_ids.len(),
            "ReleaseGroupAliasLoader batch load"
        );

        let rows = sqlx::query_as!(
            ReleaseGroupAliasRow,
            r#"SELECT
                rga.release_group,
                rga.name,
                rga.sort_name,
                at.name AS type_name,
                rga.locale,
                rga.primary_for_locale,
                rga.begin_date_year,
                rga.begin_date_month,
                rga.begin_date_day,
                rga.end_date_year,
                rga.end_date_month,
                rga.end_date_day,
                rga.ended
            FROM release_group_alias rga
            LEFT JOIN release_group_alias_type at ON at.id = rga.type
            WHERE rga.release_group = ANY($1)"#,
            release_group_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "ReleaseGroupAliasLoader query returned");

        let mut result: HashMap<i32, Vec<Alias>> = HashMap::new();
        for row in rows {
            result.entry(row.release_group).or_default().push(Alias {
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
        for id in release_group_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
