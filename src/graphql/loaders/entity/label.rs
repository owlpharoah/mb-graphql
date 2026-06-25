use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;

use crate::graphql::types::{
    common::PartialDate,
    label::{Label, LabelRow},
};

pub struct LabelLoader {
    pub pool: PgPool,
}

impl Loader<i32> for LabelLoader {
    type Value = Label;
    type Error = async_graphql::Error;

    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let rows = sqlx::query_as::<_, LabelRow>(
            r#"SELECT
                id,
                gid,
                name,
                begin_date_year,
                begin_date_month,
                begin_date_day,
                end_date_year,
                end_date_month,
                end_date_day,
                type,
                comment,
                ended,
                label_code
            FROM label
            WHERE id = ANY($1)"#,
        )
        .bind(ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    row.id,
                    Label {
                        mbid: row.gid,
                        name: row.name,
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
                        label_type: row.label_type,
                    },
                )
            })
            .collect())
    }
}
