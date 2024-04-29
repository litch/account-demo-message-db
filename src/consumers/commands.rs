// src/consumers/commands.rs

use crate::db::Store;
use crate::domain::Message;
use crate::domain::commands::{Command, Open, Close};
use axum::async_trait;

pub struct CommandsConsumer {
    store: Store,
}

impl CommandsConsumer {
    pub fn new(store: Store) -> Self {
        CommandsConsumer { store }
    }

    pub async fn start(&self, stream_name: &str) {
        self.store.subscribe_to_stream(stream_name, |message| {
            println!("Processing message: {:?}", message);
            // let command: Box<dyn Command> = self.deserialize_command(&message.data).await;
            // self.dispatch(command).await;
        }).await;
    }

    async fn deserialize_command(&self, data: &str) -> Box<dyn Command> {
        // Deserialize the JSON string into a Command

        Box::new(crate::domain::commands::Open { account_id: "123".to_string(), customer_id: "cust123".to_string() })
    }

    // async fn dispatch(&self, command: Box<dyn Command>) {
    //     let handler = crate::handlers::AccountCommandsHandler::new(self.store.clone());
    //     handler.handle(&*command).await.unwrap();
    // }
}
