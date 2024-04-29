use axum::async_trait;
use crate::messaging::Message;
use crate::messaging::Handler;
use tracing::info;

#[derive(Clone)]

pub struct AccountHandler;

impl AccountHandler {
    pub fn new() -> AccountHandler {
        AccountHandler
    }
}

#[async_trait]
impl Handler for AccountHandler {
    async fn handle(&self, message: Message) -> Result<(), String> {
        info!("Handling message of type: {}", message.message_type);

        Ok(())
    }
}