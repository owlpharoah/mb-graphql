use mb_graphql::db;
use mb_graphql::graphql;
use mb_graphql::server;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = db::create_pool().await.expect("Couldnt connect to DB");
    let schema = graphql::build_schema(pool);

    server::run(schema).await;
}
