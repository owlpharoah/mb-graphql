use mb_graphql::db;
use mb_graphql::graphql;
use mb_graphql::server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_ansi(false))
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::EnvFilter::new("info,sqlx=debug"))
        .init();

    let pool = db::create_pool().await.expect("Couldnt connect to DB");
    let schema = graphql::build_schema(pool);

    server::run(schema).await;
}
