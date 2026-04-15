use super::subscriber::Subscriber;
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

#[derive(Debug)]
pub struct Session {
    pub subscriber: Subscriber,
    pub tx: Sender<String>,
}

#[derive(Debug)]
pub struct SessionStore {
    sessions: HashMap<Uuid, Session>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn get(&self, id: Uuid) -> Option<&Session> {
        self.sessions.get(&id)
    }

    pub fn insert(&mut self, id: Uuid, session: Session) {
        self.sessions.insert(id, session);
    }

    pub fn remove(&mut self, id: &Uuid) {
        self.sessions.remove(id);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Uuid, &Session)> {
        self.sessions.iter()
    }
}

// TODO: session identifier should probably be some kind of uuid
