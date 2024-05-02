
use core::hash;

use tracing::{info};

use crate::domain::account::Account;
use crate::domain::events::{Event, Opened};
use crate::db::MessageStore;

#[derive(Clone)]
pub struct AccountStore {
    pub message_store: MessageStore,
    accounts: hash::HashMap<String, Account>,
}

impl AccountStore {

    pub async fn fetch(&self, account_id: &str) -> Result<(Account, Option<i64>), String> {
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
