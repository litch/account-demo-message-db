use serde::Serialize;
use chrono::NaiveDateTime;
use serde_json::Value;

use crate::messaging::message::Message;
use crate::messaging::commands::Command;

pub trait Event {
    fn follow(command: &dyn Command) -> Self where Self: Sized;
    fn from_message(message: Message) -> Result<Self, String> where Self: Sized;
    fn message(&self) -> &Message;
    fn event_name(&self) -> &'static str;
}

#[derive(Debug, Clone, Serialize)]
pub struct Recorded {
    pub recorded_position: i64,
    pub processed_time: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub message: Message,
    #[serde(skip_serializing)]
    pub position: Option<i64>,
}

impl Event for Recorded {
    fn follow(_command: &dyn Command) -> Self {
        // not suppported for Recorded, error if called
        panic!("Not supported for Recorded")
    }

    fn from_message(message: Message) -> Result<Self, String> {
        let data: Value = serde_json::from_str(&message.data)
            .map_err(|e| format!("Failed to parse JSON data: {}", e))?;

        let recorded_position = data["recorded_position"]
            .as_i64()
            .ok_or("Missing recorded_position in message data")?;

        let processed_time = data["processed_time"]
            .as_str()
            .map(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f").unwrap());

        let position = message.position;

        Ok(Recorded { recorded_position, processed_time, position, message })
    }

    fn message(&self) -> &Message {
        &self.message
    }

    fn event_name(&self) -> &'static str {
        "Recorded"
    }
}