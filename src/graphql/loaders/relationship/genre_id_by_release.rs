use async_graphql::dataloader::Loader;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::info;

#[derive(sqlx::FromRow)]
struct ReleaseGenreIdRow {
    release: i32,
    id: i32,
}

pub struct GenreIdsByReleaseLoader {
    pub pool: PgPool,
}

impl Loader<i32> for GenreIdsByReleaseLoader {
    type Value = Vec<i32>;
    type Error = async_graphql::Error;

    async fn load(&self, release_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        info!(
            count = release_ids.len(),
            "GenreIdsByReleaseLoader batch load"
        );

        let rows = sqlx::query_as::<_, ReleaseGenreIdRow>(
            r#"SELECT rt.release AS release, g.id AS id
            FROM release_tag rt
            JOIN tag t ON t.id = rt.tag
            JOIN genre g ON g.name = t.name
            WHERE rt.release = ANY($1)
            ORDER BY rt.release, rt.count DESC"#,
        )
        .bind(release_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        info!(rows = rows.len(), "GenreIdsByReleaseLoader query returned");

        let mut result: HashMap<i32, Vec<i32>> = HashMap::new();
        for row in rows {
            result.entry(row.release).or_default().push(row.id);
        }
        for id in release_ids {
            result.entry(*id).or_default();
        }

        Ok(result)
    }
}
