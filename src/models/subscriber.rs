use crate::models::message::Message;
use uuid::Uuid;

#[derive(Hash, Debug, Clone, Eq, PartialEq, Copy)]
pub struct Subscriber {
    pub id: Uuid,
}

impl Subscriber {
    pub fn on_message(&self, message: &Message) {
        println!("Message: {} dispatched to {}", message.content, self.id);
    }
}
