
#[derive(Debug)]
pub struct Opened {
    pub account_id: String,
    pub processed_time: String,
}

#[derive(Debug)]
pub struct Closed {
    pub account_id: String,
    pub processed_time: String,
}

#[derive(Debug)]
pub struct Withdrawn {
    pub account_id: String,
    pub amount: f64,
    pub timestamp: u64,
}