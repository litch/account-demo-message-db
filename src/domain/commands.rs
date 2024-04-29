pub trait Command {
    fn account_id(&self) -> &String;  // Provide a common method that all commands must implement
}

#[derive(Debug)]
pub struct Open {
    pub account_id: String,
    pub customer_id: String,
}

impl Command for Open {
    fn account_id(&self) -> &String {
        &self.account_id
    }
}

#[derive(Debug)]
pub struct Close {
    pub account_id: String,
}

impl Command for Close {
    fn account_id(&self) -> &String {
        &self.account_id
    }
}
