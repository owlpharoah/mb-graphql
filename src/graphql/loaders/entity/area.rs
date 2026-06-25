use crate::graphql::types::{
    area::{Area, AreaRow},
    common::PartialDate,
};
use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

pub struct AreaLoader {
    pub pool: PgPool,
}

impl Loader<i32> for AreaLoader {
    type Value = Area;
    type Error = async_graphql::Error;

    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(count = ids.len(), "AreaLoader batch load");

        let rows = sqlx::query_as::<_, AreaRow>(
            r#"SELECT
                id,
                gid,
                name,
                comment,
                type,
                ended,
                begin_date_year,
                begin_date_month,
                begin_date_day,
                end_date_year,
                end_date_month,
                end_date_day
            FROM area
            WHERE id = ANY($1);"#,
        )
        .bind(ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        info!(rows = rows.len(), "AreaLoader query returned");
        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    row.id,
                    Area {
                        mbid: row.gid,
                        name: row.name,
                        area_type: row.area_type,
                        disambiguation: row.comment,
                        ended: row.ended,
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
                        id: row.id,
                    },
                )
            })
            .collect())
    }
}
