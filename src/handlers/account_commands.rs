// // src/handlers/account_commands.rs
// use axum::async_trait;
// use crate::domain::{commands::*, events::*};
// use crate::db::Store;
// use chrono::Utc;

// pub struct AccountCommandsHandler {
//     store: Store,
// }

// impl AccountCommandsHandler {
//     pub fn new(store: Store) -> Self {
//         AccountCommandsHandler { store }
//     }

//     async fn handle_open(&self, command: Open) -> Result<(), String> {
//         let (account, version) = self.store.fetch(&command.account_id).await
//             .map_err(|_| "Failed to fetch account".to_string())?;

//         if account.is_open() {
//             // Log and ignore the command
//             println!("Command ignored: Account already open");
//             return Ok(());
//         }

//         let opened = Opened {
//             account_id: command.account_id,
//             processed_time: Utc::now().to_rfc3339(),
//         };

//         self.store.write(&opened, &format!("account:{}", opened.account_id), version).await
//     }

//     // Methods for Close, Deposit, Withdraw would be similarly implemented
// }

// #[async_trait]
// pub trait Handler {
//     async fn handle(&self, command: &dyn Command) -> Result<(), String>;
// }


// #[async_trait]
// impl Handler for AccountCommandsHandler {
//     async fn handle(&self, command: &dyn Command) -> Result<(), String> {
//         match command.account_id().as_str() {
//             "open" => self.handle_open(command.clone().downcast::<Open>().unwrap()).await,
//             "close" => self.handle_close(command.clone().downcast::<Close>().unwrap()).await,
//             _ => Err("Unknown command".to_string())
//         }
//     }
// }