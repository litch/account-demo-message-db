use crate::db::{MessageStore};
use crate::messaging::{Handler, Message, PositionStore};
use axum::async_trait;
use std::arch::global_asm;
use std::sync::Arc;
use tracing::{info, error};

#[async_trait]
pub trait Consumer {
    async fn start(&self, stream_name: &str) -> Result<(), String>;
}

#[async_trait]
pub trait StreamConsumer<T: Handler + Send + Sync + Clone + 'static>: Consumer {
    async fn process_message(&self, handler: Arc<T>, message: Message) -> Result<(), String>;
}

#[derive(Clone)]
pub struct CommandsConsumer<T: Handler + Send + Sync + Clone + 'static> {
    store: MessageStore,
    position_store: PositionStore,
    handler: T,
}

impl<T: Handler + Send + Sync + Clone + 'static> CommandsConsumer<T> {
    pub fn new(store: MessageStore, position_store: PositionStore, handler: T) -> Self {
        CommandsConsumer { store, position_store, handler }
    }
}

#[async_trait]
impl<T: Handler + Send + Sync + Clone + 'static> Consumer for CommandsConsumer<T> {
    async fn start(&self, stream_name: &str) -> Result<(), String> {
        let handler = Arc::new(self.handler.clone());
        let position_store = self.position_store.clone();

        let starting_position = position_store.get().await;

        // Assuming store.subscribe_to_stream now only requires what it absolutely needs.
        self.store.subscribe_to_stream(stream_name, starting_position, move |message| {
            let handler_clone = handler.clone();
            let position_store_clone = position_store.clone();
            let global_position = message.global_position.unwrap();
            Box::pin(async move {
                match handler_clone.handle(message).await {
                    Ok(_) => {
                        info!("Message processed successfully.");
                        if let Err(e) = position_store_clone.update_position(global_position).await {
                            error!("Failed to update position: {}", e);
                        }
                    },
                    Err(e) => error!("Failed to process message: {}", e),
                }
            })
        }).await;

        Ok(())
    }
}

#[async_trait]
impl<T: Handler + Send + Sync + Clone + 'static> StreamConsumer<T> for CommandsConsumer<T> {
    async fn process_message(&self, handler: Arc<T>, message: Message) -> Result<(), String> {
        handler.handle(message).await
    }
}
