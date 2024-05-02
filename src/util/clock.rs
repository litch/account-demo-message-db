use chrono::NaiveDateTime;

#[derive(Clone)]
pub struct Clock;

impl Clock {
    pub fn now(&self) -> NaiveDateTime {
        chrono::Utc::now().naive_utc()
    }
}
