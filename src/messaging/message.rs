// src/messaging/message.rs

use std::default;

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

impl Message {
    pub fn default() -> Self {
        Message {
            global_position: None,
            position: None,
            message_type: "".to_string(),
            data: "".to_string(),
            metadata: None,
            time: NaiveDateTime::from_timestamp(0, 0),
        }
    }
}