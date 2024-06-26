use crate::messaging::commands::Command;
use crate::messaging::events::Event;
use crate::messaging::Message;
use chrono::NaiveDateTime;
use serde_json::Value;
use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct Opened {
    pub account_id: String,
    pub processed_time: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    pub position: Option<i64>,
    #[serde(skip_serializing)]
    pub message: Message,
}

impl Event for Opened {
    fn follow(command: &dyn Command) -> Self {
        Opened {
            account_id: command.account_id().to_string(),
            processed_time: None,
            position: command.position(),
            message: command.message().clone(),
        }
    }

    fn from_message(message: Message) -> Result<Self, String> {
        let data: Value = serde_json::from_str(&message.data)
            .map_err(|e| format!("Failed to parse JSON data: {}", e))?;

        let account_id = data["account_id"]
            .as_str()
            .ok_or("Missing account_id in message data")?
            .to_string();

        let processed_time = data["processed_time"]
            .as_str()
            .map(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f").unwrap());

        let position = message.position;

        Ok(Opened { account_id, processed_time, position, message })
    }

    fn message(&self) -> &Message {
        &self.message
    }

    fn event_name(&self) -> &'static str {
        "Opened"
    }
}
