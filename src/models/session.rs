use super::subscriber::Subscriber;
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub struct Session {
    pub subscriber: Subscriber,
    pub tx: Sender<String>,
}

#[derive(Debug)]
pub struct SessionStore {
    sessions: HashMap<i32, Session>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn get(&self, id: i32) -> Option<&Session> {
        self.sessions.get(&id)
    }

    pub fn insert(&mut self, id: i32, session: Session) {
        self.sessions.insert(id, session);
    }

    pub fn remove(&mut self, id: &i32) {
        self.sessions.remove(id);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&i32, &Session)> {
        self.sessions.iter()
    }
}

// TODO: session identifier should probably be some kind of uuid
