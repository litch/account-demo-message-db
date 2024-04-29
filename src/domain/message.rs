// src/domain/message.rs

use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, FromRow)]
pub struct Message {
    pub global_position: i64,
    pub position: i64,
    pub message_type: String,
    pub data: String,
    pub metadata: Option<String>,
    pub time: NaiveDateTime,
}