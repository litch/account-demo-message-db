use crate::db::Store;
use crate::messaging::Message;

use axum::async_trait;

#[async_trait]
pub trait Consumer {
    async fn start(&self, stream_name: &str) -> Result<(), String>;
}