use axum::async_trait;
use serde::Serialize;
use crate::messaging::Message;
use crate::messaging::Handler;
use crate::db::MessageStore;

use tracing::info;

use crate::domain::commands::{Open, Close, Deposit, Withdraw, Command};
use crate::domain::events::{Event, Opened};
use crate::domain::stores::AccountStore;
use crate::util::Clock;

#[derive(Clone)]

pub struct AccountHandler {
    clock: Clock,
    account_store: AccountStore,
    message_store: MessageStore,
}

impl AccountHandler {
    pub fn new(message_store: MessageStore) -> AccountHandler {
        AccountHandler {
            clock: Clock {},
            message_store: message_store.clone(),
            account_store: AccountStore {
                message_store,
            },
        }
    }
}

#[async_trait]
impl Handler for AccountHandler {
    async fn handle(&self, message: Message) -> Result<(), String> {

        info!("Handling message of type: {}", message.message_type);
        match message.message_type.as_str() {
            "Open" => {
                let cmd = Open::from_message(message)?;
                self.handle_open(cmd).await
            },
            "Close" => {
                let cmd = Close::from_message(message)?;
                self.handle_close(cmd).await
            },
            "Deposit" => {
                let cmd = Deposit::from_message(message)?;
                self.handle_deposit(cmd).await
            },
            "Withdraw" => {
                let cmd = Withdraw::from_message(message)?;
                self.handle_withdraw(cmd).await
            },
            _ => Err("Unsupported message type".to_string()),
        }
    }
}

impl AccountHandler {
    async fn handle_open(&self, open: Open) -> Result<(), String> {
        println!("Handling Open for account: {}", open.account_id);
        let account_id = open.account_id();
        let (account, position) = self.account_store.fetch(account_id).await.expect("Failed to fetch account");
        if account.opened() {
            info!("Account already opened: {} - proceeding", account_id);
            return Ok(());
        }

        let processed_time = self.clock().now();
        let opened = Opened::follow(&open);
        let opened = Opened {
            processed_time: Some(processed_time),
            ..opened
        };
        let stream_name = format!("account-{}", account_id);
        info!("Generated Opened event: {:?}", opened);

        info!("Want to write this event to {} {}", stream_name, position.unwrap_or(0));
        self.write(&stream_name, opened, position).await.expect("Failed to write event");

        Ok(())
    }

    async fn write(&self, stream_name: &str, event: impl Event + Serialize, position: Option<i64>) -> Result<(), String> {
        info!("Writing event to stream: {}", stream_name);

        // derive the message type from the event type
        let message_type = event.event_name();
        let data = serde_json::to_value(&event).expect("Failed to serialize event").to_string();
        self.message_store.write_message(stream_name, message_type, &data, None, position).await;

        Ok(())
    }

    async fn handle_close(&self, close: Close) -> Result<(), String> {
        println!("Handling Close for account: {}", close.account_id);
        // Additional business logic for handling Close
        Ok(())
    }

    async fn handle_deposit(&self, deposit: Deposit) -> Result<(), String> {
        println!("Handling Deposit for account: {}", deposit.account_id);
        // Additional business logic for handling Deposit
        Ok(())
    }

    async fn handle_withdraw(&self, withdraw: Withdraw) -> Result<(), String> {
        println!("Handling Withdraw for account: {}", withdraw.account_id);
        // Additional business logic for handling Withdraw
        Ok(())
    }

    fn clock(&self) -> Clock {
        self.clock.clone()
    }

}
