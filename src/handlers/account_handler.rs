use axum::async_trait;
use serde::Serialize;
use crate::messaging::Message;
use crate::messaging::Handler;
use chrono::NaiveDateTime;
use crate::db::MessageStore;

use tracing::info;

use crate::domain::commands::{Open, Close, Deposit, Withdraw, Command};
use crate::domain::events::{Event, Opened};

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

        let processed_time = self.clock.now();
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
        self.message_store.write_message(stream_name, &message_type, &data, None, position).await;

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

#[derive(Clone)]
pub struct Clock;

impl Clock {
    fn now(&self) -> NaiveDateTime {
        chrono::Utc::now().naive_utc()
    }
}

pub struct Account {
    pub id: String,
    pub opened_time: Option<NaiveDateTime>,
    pub balance: Option<i64>,
    pub status: Option<String>,
}

impl Account {
    pub fn new(id: &str) -> Account {
        Account {
            id: id.to_string(),
            opened_time: None,
            balance: None,
            status: None,
        }
    }

    pub fn opened(&self) -> bool {
        self.opened_time.is_some()
    }
}

#[derive(Clone)]
pub struct AccountStore {
    message_store: MessageStore,
}

impl AccountStore {

    async fn fetch(&self, account_id: &str) -> Result<(Account, Option<i64>), String> {
        info!("Fetching account: {}", account_id);
        let messages = self.message_store.get_stream_messages(&format!("account-{}", account_id), None, None, None).await.expect("Failed to fetch messages");

        let mut account = Account::new(account_id);
        let mut position = None;
        for message in messages {
            info!("Processing account message: {:?}", message);
            let message_position = message.position;
            match message.message_type.as_str() {
                "Opened" => {
                    let event = Opened::from_message(message)?;
                    account = self.apply_opened(account, event);
                },
                _ => (),
            }
            position = message_position;
        }

        Ok((account, position))
    }

    fn apply_opened(&self, account: Account, opened: Opened) -> Account {
        println!("Applying Opened event to account: {:?}", opened);
        Account {
            id: account.id,
            opened_time: opened.processed_time,
            balance: None,
            status: None,
        }
    }
}
