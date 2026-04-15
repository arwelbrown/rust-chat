use crate::models::publisher::Publisher;
use rusqlite::{Connection, Result};
use std::{fs, path::Path, str, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct Utils {
    i: String,
}

impl Utils {
    pub fn run_migrations(conn: &Connection) -> Result<()> {
        if !Path::new("../sql/001_seed.sql").exists() {
            return Ok(());
        }

        let sql = fs::read_to_string("../sql/001_seed.sql").expect("Failed to read seed.sql");
        conn.execute_batch(&sql)?;

        Ok(())
    }

    pub fn formatter(
        msg: &str,
        _publisher: Arc<Mutex<Publisher>>,
    ) -> Result<(Uuid, Uuid, String), anyhow::Error> {
        let msg = msg.trim_end();
        let parts: Vec<&str> = msg.split('|').collect();

        if parts.len() != 3 {
            return Err(anyhow::anyhow!(
                "Invalid message format: {}. Did you supply all required fields?",
                msg
            ));
        }

        let conversation_id = parts[0]
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid conversation_id: {}", parts[0]))?;

        let user_id = parts[1]
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid user_id: {}", parts[1]))?;

        let message = parts[2].to_string();

        println!(
            "Parsed handshake: conversation_id: {}, user_id: {}, message: {}",
            conversation_id, user_id, message
        );

        Ok((conversation_id, user_id, message))
    }
}
