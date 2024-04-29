// src/domain/mod.rs

pub mod message;
pub mod account;
pub mod events;
pub mod commands;

// Optionally re-export key components for easier use elsewhere in the project
pub use message::Message;
pub use account::Account;
pub use events::Withdrawn;
pub use events::Opened;
pub use events::Closed;
pub use commands::Open;
pub use commands::Close;