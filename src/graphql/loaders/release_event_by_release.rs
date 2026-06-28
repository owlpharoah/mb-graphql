use crate::graphql::types::common::{PartialDate, ReleaseEvent};
use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct ReleaseReleaseEventRow {
    date_year: Option<i16>,
    date_month: Option<i16>,
    date_day: Option<i16>,
    country: Option<i32>,
    release: i32,
}

pub struct ReleaseEventsByReleaseLoader {
    pub pool: PgPool,
}

impl Loader<i32> for ReleaseEventsByReleaseLoader {
    type Value = Vec<ReleaseEvent>;
    type Error = async_graphql::Error;

    async fn load(&self, release_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = release_ids.len(),
            "ReleaseEventsByReleaseLoader batch load"
        );
        let rows = sqlx::query_as!(
            ReleaseReleaseEventRow,
            r#"
                SELECT
                    release AS "release!",
                    date_year,
                    date_month,
                    date_day,
                    country
                FROM release_event
                WHERE release = ANY($1)
                "#,
            release_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(
            rows = rows.len(),
            "ReleaseEventsByReleaseLoader query returned"
        );
        let mut result: HashMap<i32, Vec<ReleaseEvent>> = HashMap::new();
        for row in rows {
            result.entry(row.release).or_default().push(ReleaseEvent {
                date: PartialDate::from_parts(row.date_year, row.date_month, row.date_day),
                country: row.country,
            });
        }
        for id in release_ids {
            result.entry(*id).or_default();
        }
        Ok(result)
    }
}
