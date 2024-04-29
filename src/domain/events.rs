use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use axum::async_trait;

pub trait Message {
    fn message_type(&self) -> String;
    fn message_name(&self) -> String;
}

// Metadata structure, similar to Ruby's metadata handling
#[derive(Default, Debug, Clone)]
pub struct Metadata {
    correlation_stream_name: Option<String>,
}

// Implement functions that would be used to manage message metadata and types
pub fn build_metadata() -> Metadata {
    Metadata::default()
}

pub trait Event {
    fn account_id(&self) -> &str;
    fn message_type(&self) -> &str;
    fn to_json(&self) -> String;
}


#[derive(Debug)]
struct Deposit {
    account_id: String,
    amount: f64,
    time: String,
}

impl Event for Deposit {
    fn account_id(&self) -> &str {
        &self.account_id
    }

    fn message_type(&self) -> &str {
        "Deposit"
    }

    fn to_json(&self) -> String {
        format!("{{\"account_id\": \"{}\", \"amount\": {}, \"time\": \"{}\"}}", self.account_id, self.amount, self.time)
    }
}
