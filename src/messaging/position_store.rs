use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, debug};

use crate::db::MessageStore;
use crate::util::Clock;
use crate::messaging::Message;
use crate::messaging::events::{Event, Recorded};

#[derive(Clone)]
pub struct PositionStore {
    pub message_store: MessageStore,
    pub category: String,
    pub identifier: Option<String>,
    pub position: Arc<Mutex<i64>>,
    clock: Clock,
}

impl PositionStore {
    pub fn new(message_store: MessageStore, category: String, identifier: Option<String>) -> Self {
        PositionStore {
            message_store,
            category,
            identifier,
            position: Arc::new(Mutex::new(0)),  // Start with initial position of 0
            clock: Clock {},
        }
    }

    pub fn position_stream_name(&self) -> String {
        if let Some(ref id) = self.identifier {
            format!("{}:position-{}", self.category, id)
        } else {
            format!("{}:position", self.category)
        }
    }

    pub async fn get(&self) -> i64 {
        let message = self.message_store.get_last_message(&self.position_stream_name()).await.unwrap();
        debug!("Getting position for stream {:?}, last message: {:?}", self.position_stream_name(), message);
        match message {
            Some(message) => {
                let event = Recorded::from_message(message).expect("Failed to parse event");
                event.recorded_position
            },
            None => 0,
        }
    }

    pub async fn position(&self, account_id: &str) -> Result<i64, String> {
        info!("Fetching position for account: {}", account_id);
        let position = self.position.lock().await;
        Ok(*position)
    }

    pub async fn update_position(&self, new_position: i64) -> Result<(), String> {
        info!("Updating position to: {}", new_position);
        let mut position = self.position.lock().await;
        *position = new_position;

        if *position % 5 == 0 {
            self.save_position(*position).await?;
        }

        Ok(())
    }

    async fn save_position(&self, position: i64) -> Result<(), String> {

        info!("Saving position to the database: {}", position);
        let now = self.clock.now();
        let event = Recorded {
            recorded_position: position,
            processed_time: Some(now),
            position: None,
            message: Message::default(),

         };
        let message_type = event.event_name();
        let data = serde_json::to_value(&event).expect("Failed to serialize event").to_string();
        self.message_store.write_message(&self.position_stream_name(), message_type, &data, None, None).await;

        Ok(())
    }
}


