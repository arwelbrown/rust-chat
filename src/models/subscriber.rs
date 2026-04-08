use crate::models::message::Message;

#[derive(Hash, Debug, Clone, Eq, PartialEq, Copy)]
pub struct Subscriber {
    pub id: i32,
}

impl Subscriber {
    pub fn on_message(&self, message: &Message) {
        println!("Message: {} dispatched to {}", message.content, self.id);
    }
}
