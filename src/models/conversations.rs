use crate::models::{message::Message, subscriber::Subscriber};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ChatRoom {
    pub id: Uuid,
    pub subscribers: HashSet<Subscriber>,
    pub messages: Vec<Message>,
}

impl ChatRoom {
    pub fn new(id: Uuid) -> Self {
        ChatRoom {
            id: id,
            subscribers: HashSet::new(),
            messages: Vec::new(),
        }
    }
}
