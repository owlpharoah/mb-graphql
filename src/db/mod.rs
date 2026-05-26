use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").expect("Database URL must be set in env");

    PgPoolOptions::new()
        .max_connections(20)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(600))
        .connect(&db_url)
        .await
}
