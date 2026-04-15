use crate::models::{
    conversations::ChatRoom, message::Message, session::SessionStore, sqlite::SqLite,
    subscriber::Subscriber,
};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

// Publisher is responsible for managing chat rooms and conversations.
pub struct Publisher {
    pub conversations: HashMap<Uuid, ChatRoom>, // registry of subscribers/listeners
    pub db: SqLite,
}

impl Publisher {
    pub fn create_room(&mut self, room: ChatRoom) {
        self.conversations
            .insert(room.id.to_owned(), room.to_owned());

        // TODO: persist room to db
    }

    pub fn sub_to_room(&mut self, conversation_id: Uuid, subscribers: HashSet<Subscriber>) {
        if let Some(existing_conversation) = self.conversations.get_mut(&conversation_id) {
            for s in &subscribers {
                existing_conversation.subscribers.insert(s.to_owned());
            }
        } else {
            let conversation = ChatRoom {
                id: conversation_id,
                subscribers,
                messages: Vec::new(),
            };

            self.create_room(conversation);
        }
    }

    pub fn unsub_from_room(&mut self, conversation_id: Uuid, subscribers: HashSet<Subscriber>) {
        if let Some(existing_members) = self.conversations.get_mut(&conversation_id) {
            for s in &subscribers {
                existing_members.subscribers.remove(s);
            }
        }

        // TODO: after room persistence feature is complete, delete if room is empty
    }

    pub async fn dispatch_messages(
        &mut self,
        conversation_id: Uuid,
        message: &Message,
        store: Arc<Mutex<SessionStore>>,
    ) {
        if let Some(conversation) = self.conversations.get_mut(&conversation_id) {
            conversation.messages.push(message.clone());

            let sessions = store.lock().await;
            for sub in &conversation.subscribers {
                if let Some(session) = sessions.get(sub.id) {
                    if let Err(e) = session.tx.send(message.content.clone()).await {
                        eprintln!("Failed to send message to subscriber {}: {}", sub.id, e);
                    }
                }
            }
        }
    }

    pub fn delete_room(&mut self, conversation_id: Uuid) {
        self.conversations.remove(&conversation_id);

        println!("Chat room {} deleted", conversation_id);
    }

    pub fn list_rooms(&self) {
        for (id, conversation) in self.conversations.iter() {
            println!(
                "Chat Room {}  -> subscribers {:?} messages {:?} ",
                id, conversation.subscribers, conversation.messages
            )
        }
    }
}
