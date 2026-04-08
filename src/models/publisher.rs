use crate::models::{conversations::ChatRoom, sqlite::SqLite, subscriber::Subscriber};
use std::collections::{HashMap, HashSet};

// Publisher is responsible for managing chat rooms and conversations.
pub struct Publisher {
    pub conversations: HashMap<i32, ChatRoom>, // registry of subscribers/listeners
    pub db: SqLite,
}

impl Publisher {
    pub fn create_room(&mut self, room: ChatRoom) {
        self.conversations
            .insert(room.id.to_owned(), room.to_owned());
    }

    pub fn sub_to_room(&mut self, conversation_id: i32, subscribers: HashSet<Subscriber>) {
        if let Some(existing_conversation) = self.conversations.get_mut(&conversation_id) {
            for s in &subscribers {
                existing_conversation.subscribers.insert(s.to_owned());
            }
        }
    }
}
