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

mod db;

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

    // Assuming message parameters are defined or retrieved from somewhere

    let handler = Consumer {
        name: "Consumer1".to_string(),
        stream: "decommission:commands".to_string(),
    };

    subscribe_to_stream(&db, stream_name, handler).await;

}
// Define the Handler trait with async capabilities
#[async_trait]
pub trait Handler {
    async fn handle(&self, message: Message);
}

// Implement the Consumer struct that will implement the Handler trait
pub struct Consumer {
    name: String,
}

#[async_trait]
impl Handler for Consumer {
    async fn handle(&self, message: Message) {
        info!("Consumer {} handling message: {:?}", self.name, message);
    }
}

// Generic function to subscribe to a stream and handle messages using a specified handler
pub async fn subscribe_to_stream<T: Handler + Send + Sync>(
    db: &db::Db,
    stream_name: &str,
    handler: T,
) {
    let mut position = 0;
    // Assuming get_stream_messages is an async function that retrieves messages from a database
    match get_stream_messages(db, stream_name, None, None, None).await {
        Ok(messages) => {
            for message in messages {
                debug!("Message: {:?}", message);
                position = message.position;
                handler.handle(message).await;

            }
        },
        Err(e) => error!("Failed to fetch messages: {}", e),
    }
    debug!("Initial replay of stream completed");
    // Position will be retrieved from the last message processed
    loop {
        match get_stream_messages(db, stream_name, Some(position+1), None, None).await {
            Ok(messages) => {
                for message in messages {
                    debug!("Message: {:?}", message);
                    position = message.position;
                    handler.handle(message).await;
                }
                // sleep a few seconds before fetching more messages
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            },
            Err(e) => error!("Failed to fetch messages: {}", e),
        }
    }
}



#[instrument(skip(db))]
async fn write_message(
    db: &db::Db,
    stream_name: &str,
    message_type: &str,
    data: &str,
    metadata: Option<&str>,  // Optional, can be None
    expected_version: Option<i64>  // Optional, can be None for new streams or first message
) {

    let message_id = uuid::Uuid::new_v4();
    let query = r#"
        SELECT write_message($1::varchar, $2::varchar, $3::varchar, $4::jsonb, $5::jsonb, $6::bigint);
    "#;
    let result = sqlx::query(query)
        .bind(message_id.to_string())
        .bind(stream_name)
        .bind(message_type)
        .bind(data)
        .bind(metadata.unwrap_or("null"))
        .bind(expected_version)
        .execute(db.pool())
        .await;

    match result {
        Ok(_) => info!("Message written successfully"),
        Err(e) => error!("Failed to write message: {}", e),
    }
}

#[derive(Debug, sqlx::FromRow)]
struct Message {
    global_position: i64,
    position: i64,
    data: String,  // Changed from serde_json::Value to String
    metadata: Option<String>,  // Optional metadata
    time: chrono::NaiveDateTime,  // Using NaiveDateTime, adjust based on your timezone handling.
}
