use crate::domain::events::Event;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    id: String,
    balance: i64,
    state: AccountState,
}

impl Account {
    pub fn new(id: String) -> Self {
        Account { id, balance: 0, state: AccountState::New }
    }

    // pub fn apply_event(&mut self, event: &Event) {
    //     match event {
    //         // Event::AccountOpened { .. } => self.state = AccountState::Opened,
    //         // Event::MoneyDeposited { amount } => self.balance += amount,
    //         // Event::MoneyWithdrawn { amount } => self.balance -= amount,
    //         _ => {}
    //     }
    // }
}

#[derive(Serialize, Deserialize, Debug)]
enum AccountState {
    New,
    Opened,
    Closed,
}
