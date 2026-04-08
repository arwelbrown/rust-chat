use crate::models::{message::Message, subscriber::Subscriber};
use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ChatRoom {
    pub id: i32, // maybe should be a uuid?
    pub subscribers: HashSet<Subscriber>,
    pub messages: Vec<Message>,
}

impl ChatRoom {
    pub fn new(id: i32) -> Self {
        ChatRoom {
            id: id,
            subscribers: HashSet::new(),
            messages: Vec::new(),
        }
    }
}
