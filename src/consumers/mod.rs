// src/consumers/mod.rs

pub mod commands;

// Re-export for easier access from outside this module
pub use commands::CommandsConsumer;