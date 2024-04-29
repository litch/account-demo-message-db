use dotenv::dotenv;
use std::env;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use handlers::AccountHandler;
use messaging::Consumer;

mod db;
mod domain;
mod consumers;
mod handlers;
mod messaging;

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

    let db = db::Db::new(&database_url).await
        .expect("Failed to create database connection pool");

    let message_store = db::MessageStore::new(db);
    let handler = AccountHandler::new();
    let commands_consumer = consumers::CommandsConsumer::new(message_store, handler);
    let _ = commands_consumer.start("account:commands").await;

}


