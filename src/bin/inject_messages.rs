use dotenv::dotenv;
use axum::async_trait;

use std::env;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use sqlx::Executor;
use sqlx::postgres::PgRow;
use sqlx::Row;

use tracing::{error, info, instrument, debug};
use uuid;
use serde_json;
use chrono;

use account_demo::db::{Store, Db};

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

    let store = Store::new(db);

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


