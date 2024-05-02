use dotenv::dotenv;
use std::env;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use handlers::AccountHandler;
use messaging::Consumer;

mod db;
mod domain;
mod handlers;
mod messaging;
mod util;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let db = db::Db::new(&database_url).await
        .expect("Failed to create database connection pool");

    let message_store = db::MessageStore::new(db);
    let handler = AccountHandler::new(message_store.clone());
    let position_store = messaging::PositionStore::new(message_store.clone(), "account:commands".to_string(), None);
    let account_consumer = messaging::CommandsConsumer::new(message_store, position_store, handler);
    let _ = account_consumer.start("account:commands").await;

}


