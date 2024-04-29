// src/messaging/message.rs

use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, FromRow, Clone)]
pub struct Message {
    pub global_position: Option<i64>,
    pub position: Option<i64>,
    pub message_type: String,
    pub data: String,
    pub metadata: Option<String>,
    pub time: NaiveDateTime,
}