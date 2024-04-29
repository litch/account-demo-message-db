

use std::pin::Pin;
use std::future::Future;

use tracing::{error, info, instrument, debug};

use crate::db;
use crate::messaging::message::Message;

#[derive(Debug)]
pub struct MessageStore {
    db: db::Db,
}

impl MessageStore {
    pub fn new(db: db::Db) -> Self {
        Self { db }
    }

    pub async fn subscribe_to_stream<F>(
        &self,
        stream_name: &str,
        mut f: F,
    ) where
        F: FnMut(Message) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + 'static,
    {
        let mut last_position = 0;
        loop {
            let messages = self.get_category_messages(stream_name, Some(last_position+1), None, None, None, None, None).await;
            match messages {
                Ok(messages) if !messages.is_empty() => {
                    for message in messages {
                        last_position = message.global_position.unwrap_or(last_position);
                        debug!("Dispatching message with position {}: {:?}", last_position, message);
                        f(message).await;
                        debug!("Message with position {} handled successfully", last_position);
                    }
                },
                Ok(_) => {
                    debug!("No new messages at position {}", last_position);
                },
                Err(e) => {
                    error!("Failed to fetch messages: {}", e);
                    break; // or handle error appropriately
                }
            }
            // sleep a few seconds
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }


    #[instrument]
    async fn get_stream_messages(
        &self,
        stream_name: &str,
        position: Option<i64>,
        batch_size: Option<i64>,
        condition: Option<&str>
    ) -> Result<Vec<Message>, sqlx::Error> {
        if condition.is_some() {
            error!("Condition is not supported for category messages");
            return Err(sqlx::Error::Protocol("Condition is not supported for category messages".to_string()));
        }
        let db = &self.db;
        // Set the search path to include message_store
        sqlx::query("SET search_path TO message_store, public;")
            .execute(db.pool())
            .await?;

        let query = r#"
            SELECT global_position, position, type AS message_type, data, metadata, time
            FROM get_stream_messages($1, $2, $3, NULL);
        "#;

        let messages = sqlx::query_as::<_, Message>(query)
            .bind(stream_name)
            .bind(position.unwrap_or(0))
            .bind(batch_size.unwrap_or(1000))
            .fetch_all(db.pool())
            .await;

        match messages {
            Ok(messages) => {
                info!("Messages fetched successfully.");
                Ok(messages)
            },
            Err(e) => {
                error!("Failed to fetch messages: {}", e);
                Err(e)
            }
        }
    }


    #[instrument]
    async fn get_category_messages(
        &self,
        category_name: &str,
        position: Option<i64>,
        batch_size: Option<i64>,
        correlation: Option<&str>,
        consumer_group_member: Option<i64>,
        consumer_group_size: Option<i64>,
        condition: Option<&str>
    ) -> Result<Vec<Message>, sqlx::Error> {
        if condition.is_some() {
            error!("Condition is not supported for category messages");
            return Err(sqlx::Error::Protocol("Condition is not supported for category messages".to_string()));
        }
        let db = &self.db;
        let query = r#"
            SELECT global_position, position, type AS message_type, data, metadata, time
            FROM get_category_messages($1, $2, $3, $4, $5, $6, NULL);
        "#;

        let messages = sqlx::query_as::<_, Message>(query)
            .bind(category_name)
            .bind(position.unwrap_or(0))  // Default to 0 if None
            .bind(batch_size.unwrap_or(1000))  // Default to 1000 if None
            .bind(correlation)  // Properly pass None as SQL NULL
            .bind(consumer_group_member)
            .bind(consumer_group_size)

            .fetch_all(db.pool())
            .await;

        match messages {
            Ok(messages) => {
                info!("Category messages fetched successfully.");
                Ok(messages)
            },
            Err(e) => {
                error!("Failed to fetch category messages: {}", e);
                Err(e)
            }
        }
    }

    #[instrument]
    pub async fn write_message(
        &self,
        stream_name: &str,
        message_type: &str,
        data: &str,
        metadata: Option<&str>,  // Optional, can be None
        expected_version: Option<i64>  // Optional, can be None for new streams or first message
    ) {
        let db = &self.db;
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
}
