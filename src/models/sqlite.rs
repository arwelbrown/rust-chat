use crate::{
    models::{conversations::ChatRoom, message::Message},
    utils::utils::Utils,
};

use rusqlite::{Connection, Result};
use uuid::Uuid;

pub struct SqLite {
    conn: Connection,
    db_name: String,
}

impl SqLite {
    pub fn init(db_name: String) -> Result<Self> {
        let conn = Connection::open(&db_name)?;
        println!("Connected to {}", &db_name);

        Utils::run_migrations(&conn)?;

        Ok(Self { conn, db_name })
    }

    pub fn add_message(&self, msg: Message) -> Result<()> {
        self.conn.execute(
            "INSERT INTO messages (conversation_id, sender_id, content, timestamp, status) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                msg.conversation_id.to_string(),
                msg.sender_id.to_string(),
                &msg.content,
                &msg.timestamp,
                &format!("{:?}", msg.status),
            ),
        )?;
        Ok(())
    }

    pub fn add_subscriber(&self, conversation_id: i32, subscriber_id: i32) -> Result<()> {
        self.conn.execute(
            "INSERT INTO chat_room_subscribers (conversation_id, subscriber_id) VALUES (?1, ?2)",
            (conversation_id, subscriber_id),
        )?;
        Ok(())
    }

    pub fn save_new_chat_room(&self, chat_room: ChatRoom) -> Result<()> {
        self.conn.execute(
            "INSERT INTO chat_room (chat_room_id) VALUES (?1)",
            (chat_room.id.to_string(),),
        )?;
        Ok(())
    }

    pub fn delete_chat_room(&self, chat_room: Uuid) -> Result<()> {
        self.conn.execute(
            "DELETE FROM chat_room WHERE chat_room_id = ?1",
            (chat_room.to_string(),),
        )?;
        Ok(())
    }

    pub fn remove_subscriber(&self, chat_room_id: Uuid, subscriber_id: Uuid) -> Result<()> {
        self.conn.execute(
            "DELETE FROM chat_room_subscribers WHERE chat_room_id = ?1 AND subscriber_id = ?2",
            (chat_room_id.to_string(), subscriber_id.to_string()),
        )?;
        Ok(())
    }
}
