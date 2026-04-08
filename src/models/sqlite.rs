use crate::models::{conversations::ChatRoom, message::Message};

use rusqlite::{Connection, Result};

pub struct SqLite {
    conn: Connection,
    db_name: String,
}

impl SqLite {
    pub fn init(db_name: String) -> Result<Self> {
        let conn = Connection::open(&db_name)?;
        println!("Connected to {}", db_name);
        Ok(Self { conn, db_name })
    }

    pub fn add_message(&self, msg: Message) -> Result<()> {
        self.conn.execute(
            "INSERT INTO messages (conversation_id, sender_id, content, timestamp, status) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                msg.conversation_id,
                msg.sender_id,
                &msg.content,
                &msg.timestamp,
                &format!("{:?}", msg.status),
            ),
        )?;
        Ok(())
    }
}
