pub trait Command {
    fn account_id(&self) -> &str;
}

// Implement specific commands
#[derive(Debug)]
pub struct Open {
    account_id: String,
    customer_id: String,
}

impl Command for Open {
    fn account_id(&self) -> &str {
        &self.account_id
    }
}

#[derive(Debug)]
pub struct Close {
    account_id: String,
}

impl Command for Close {
    fn account_id(&self) -> &str {
        &self.account_id
    }
}

#[derive(Debug)]
struct Deposit {
    account_id: String,
    deposit_id: String,
    amount: f64,
}

impl Command for Deposit {
    fn account_id(&self) -> &str {
        &self.account_id
    }
}

#[derive(Debug)]
struct Withdraw {
    account_id: String,
    withdrawal_id: String,
    amount: f64,
}

impl Command for Withdraw {
    fn account_id(&self) -> &str {
        &self.account_id
    }
}
