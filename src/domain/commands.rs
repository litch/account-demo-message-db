
use crate::messaging::Message;
use serde_json::Value;
use std::fmt;

pub trait Command {
    fn from_message(message: Message) -> Result<Self, String> where Self: Sized;
    fn account_id(&self) -> &str;
    fn position(&self) -> Option<i64>;
    fn message(&self) -> &Message;
}

#[derive(Debug, Clone)]
pub struct Open {
    pub account_id: String,
    pub message: Message,
}

impl Command for Open {
    fn from_message(message: Message) -> Result<Self, String> {
        let data: Value = serde_json::from_str(&message.data)
            .map_err(|e| format!("Failed to parse JSON data: {}", e))?;

        let account_id = data["account_id"]
            .as_str()
            .ok_or("Missing account_id in message data")?
            .to_string();

        Ok(Open { account_id, message })
    }

    fn account_id(&self) -> &str {
        &self.account_id
    }

    fn position(&self) -> Option<i64> {
        self.message.position
    }

    fn message(&self) -> &Message {
        &self.message
    }
}


#[derive(Debug, Clone)]
pub struct Close {
    pub account_id: String,
    pub message: Message,
}

impl Command for Close {
    fn from_message(message: Message) -> Result<Self, String> {
        let data: Value = serde_json::from_str(&message.data)
            .map_err(|e| format!("Failed to parse JSON data: {}", e))?;

        let account_id = data["account_id"]
            .as_str()
            .ok_or("Missing account_id in message data")?
            .to_string();

        Ok(Close { account_id, message })
    }

    fn account_id(&self) -> &str {
        &self.account_id
    }

    fn position(&self) -> Option<i64> {
        self.message.position
    }

    fn message(&self) -> &Message {
        &self.message
    }
}

#[derive(Debug, Clone)]
pub struct Deposit {
    pub account_id: String,
    pub amount: f64,
    pub message: Message,
}

impl Command for Deposit {
    fn from_message(message: Message) -> Result<Self, String> {
        let data: Value = serde_json::from_str(&message.data)
            .map_err(|e| format!("Failed to parse JSON data: {}", e))?;

        let account_id = data["account_id"]
            .as_str()
            .ok_or("Missing account_id in message data")?
            .to_string();

        let amount = data["amount"]
            .as_f64()
            .ok_or("Missing amount in message data")?;

        Ok(Deposit { account_id, amount, message })
    }

    fn account_id(&self) -> &str {
        &self.account_id
    }

    fn position(&self) -> Option<i64> {
        self.message.position
    }

    fn message(&self) -> &Message {
        &self.message
    }
}

#[derive(Debug, Clone)]
pub struct Withdraw {
    pub account_id: String,
    pub amount: f64,
    pub message: Message,
}

impl Command for Withdraw {
    fn from_message(message: Message) -> Result<Self, String> {
        let data: Value = serde_json::from_str(&message.data)
            .map_err(|e| format!("Failed to parse JSON data: {}", e))?;

        let account_id = data["account_id"]
            .as_str()
            .ok_or("Missing account_id in message data")?
            .to_string();

        let amount = data["amount"]
            .as_f64()
            .ok_or("Missing amount in message data")?;

        Ok(Withdraw { account_id, amount, message })
    }

    fn account_id(&self) -> &str {
        &self.account_id
    }

    fn position(&self) -> Option<i64> {
        self.message.position
    }

    fn message(&self) -> &Message {
        &self.message
    }
}