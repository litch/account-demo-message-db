use chrono::NaiveDateTime;

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
