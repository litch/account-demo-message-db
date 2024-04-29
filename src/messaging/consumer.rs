


use axum::async_trait;

#[async_trait]
pub trait Consumer {
    async fn start(&self, stream_name: &str) -> Result<(), String>;
}