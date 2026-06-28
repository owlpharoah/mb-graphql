use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct ReleaseGroupGenreIdRow {
    release_group: i32,
    id: i32,
}

pub struct GenreIdsByReleaseGroupLoader {
    pub pool: PgPool,
}

impl Loader<i32> for GenreIdsByReleaseGroupLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(
        &self,
        release_group_ids: &[i32],
    ) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = release_group_ids.len(),
            "GenreIdsByReleaseGroupLoader batch load"
        );

        let rows = sqlx::query_as!(
            ReleaseGroupGenreIdRow,
            r#"SELECT rgt.release_group AS release_group, g.id AS id
            FROM release_group_tag rgt
            JOIN tag t ON t.id = rgt.tag
            JOIN genre g ON g.name = t.name
            WHERE rgt.release_group = ANY($1)
            ORDER BY rgt.release_group, rgt.count DESC"#,
            release_group_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(
            rows = rows.len(),
            "GenreIdsByReleaseGroupLoader query returned"
        );

        let mut result: HashMap<i32, Vec<i32>> = HashMap::new();
        for row in rows {
            result.entry(row.release_group).or_default().push(row.id);
        }
        for id in release_group_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
