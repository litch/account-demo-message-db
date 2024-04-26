#[derive(Debug, Clone)]
struct Withdraw {
    account_id: String,
    amount: f64,
    time: String,
}

#[derive(Debug, Clone)]
struct Withdrawn {
    account_id: String,
    amount: f64,
    time: String,
    processed_time: String,
}

#[derive(Debug)]
struct Account {
    id: String,
    balance: f64,
}

impl Account {
    fn new(id: String) -> Self {
        Account { id, balance: 0.0 }
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if self.sufficient_funds(amount) {
            self.balance -= amount;
            true
        } else {
            false
        }
    }

    fn sufficient_funds(&self, amount: f64) -> bool {
        self.balance >= amount
    }
}
