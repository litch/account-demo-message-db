use dotenv::dotenv;


use std::env;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;









use account_demo::db::{MessageStore, Db};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let db = Db::new(&database_url).await
        .expect("Failed to create database connection pool");

    let store = MessageStore::new(db);

    let data = serde_json::json!({
        "account_id": uuid::Uuid::new_v4().to_string(),
        "name": "Alice",
    }).to_string();

    store.write_message(
        "account:commands",
        "Open",
        &data,
        None,
        None).await;
}


