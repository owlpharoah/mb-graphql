use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

use crate::graphql::types::common::{Alias, PartialDate};

#[derive(sqlx::FromRow)]
struct RecordingAliasRow {
    recording: i32,
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

pub struct RecordingAliasLoader {
    pub pool: PgPool,
}

impl Loader<i32> for RecordingAliasLoader {
    type Value = Vec<Alias>;
    type Error = async_graphql::Error;

    async fn load(&self, recording_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = recording_ids.len(),
            "RecordingAliasLoader batch load"
        );

        let rows = sqlx::query_as!(
            RecordingAliasRow,
            r#"SELECT
                rca.recording,
                rca.name,
                rca.sort_name,
                at.name As type_name,
                rca.locale,
                rca.primary_for_locale,
                rca.begin_date_year,
                rca.begin_date_month,
                rca.begin_date_day,
                rca.end_date_year,
                rca.end_date_month,
                rca.end_date_day,
                rca.ended
            FROM recording_alias rca
            LEFT JOIN recording_alias_type at ON at.id = rca."type"
            WHERE rca.recording = ANY($1)"#,
            recording_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "RecordingAliasLoader query returned");

        let mut result: HashMap<i32, Vec<Alias>> = HashMap::new();
        for row in rows {
            result.entry(row.recording).or_default().push(Alias {
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
        for id in recording_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
