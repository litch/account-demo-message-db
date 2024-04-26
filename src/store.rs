use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, FromRow)]
struct Message {
    global_position: i64,
    position: i64,
    data: String,
    metadata: Option<String>,
    time: NaiveDateTime,
}

pub struct Store {
    db: Db,
}

impl Store {
    pub fn new(db: Db) -> Self {
        Self { db }
    }


    #[instrument(skip(db))]
    async fn get_stream_messages(
        db: &db::Db,
        stream_name: &str,
        position: Option<i64>,
        batch_size: Option<i64>,
        condition: Option<&str>
    ) -> Result<Vec<Message>, sqlx::Error> {
        // Set the search path to include message_store
        sqlx::query("SET search_path TO message_store, public;")
            .execute(db.pool())
            .await?;

        let query = r#"
            SELECT global_position, position, data, metadata, time
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


    #[instrument(skip(db))]
    async fn get_category_messages(
        db: &db::Db,
        category_name: &str,
        position: Option<i64>,
        batch_size: Option<i64>,
        correlation: Option<&str>,
        consumer_group_member: Option<i64>,
        consumer_group_size: Option<i64>,
        condition: Option<&str>
    ) -> Result<Vec<Message>, sqlx::Error> {
        let query = r#"
            SELECT global_position, position, data, metadata, time
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

}
