use crate::messaging::Message;

pub trait Command {
    fn from_message(message: Message) -> Result<Self, String> where Self: Sized;
    fn account_id(&self) -> &str;
    fn position(&self) -> Option<i64>;
    fn message(&self) -> &Message;
}
