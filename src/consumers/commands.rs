// src/consumers/commands.rs

use crate::db::Store;
use crate::messaging::Handler;
use crate::messaging::Consumer;
use axum::async_trait;
use tracing::{info, error};
use std::sync::Arc;

pub struct CommandsConsumer<T: Handler> {

    store: Store,
    handler: T,
}

impl<T: Handler> CommandsConsumer<T> {
    pub fn new(store: Store, handler: T) -> Self {
        CommandsConsumer { store, handler }
    }
}

#[async_trait]
impl<T: Handler + Send + Sync + Clone + 'static> Consumer for CommandsConsumer<T> {
    async fn start(&self, stream_name: &str) -> Result<(), String> {
        let handler = Arc::new(self.handler.clone());
        self.store.subscribe_to_stream(stream_name, move |message| {
            let handler_clone = handler.clone();
            Box::pin(async move {
                let message_type = message.message_type.clone();
                if let Err(e) = handler_clone.handle(message).await {
                    error!("Failed to handle message: {}", e);
                } else {
                    info!("Message handled successfully: {}", message_type);
                }
            })
        }).await;

        Ok(())
    }
}