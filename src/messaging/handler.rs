use axum::async_trait;

use crate::messaging::Message;

#[async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, message: Message) -> Result<(), String>;

}
