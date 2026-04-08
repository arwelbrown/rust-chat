pub mod models;
use std::sync::Arc;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, tcp::OwnedReadHalf},
    sync::Mutex,
};

use crate::models::{publisher::Publisher, session::SessionStore};

const PORT: &str = "7878";

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT)).await?;

    println!("Server listening on port {}...", PORT);

    loop {
        let (tcp_stream, socket_addr) = match listener.accept().await {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Failed to accept connection {}", e);
                continue;
            }
        };

        println!("New connection from: {}", socket_addr);

        // split stream into read and write halves
        let (read, mut write) = tcp_stream.into_split();
        let mut reader: BufReader<OwnedReadHalf> = BufReader::new(read);
        let mut first_line = String::new();

        let n = match reader.read_line(&mut first_line).await {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read handshake from {}: {}", socket_addr, e);
                continue;
            }
        };

        if n == 0 {
            println!("Read 0 bytes, connection closed");
        }

        let msg = first_line.trim_end().to_string();

        write.write_all(msg.as_bytes()).await?;
    }
}

async fn reader_loop(
    mut reader: BufReader<OwnedReadHalf>,
    subscriber_id: i32,
    conversation_id: i32,
    publisher: Arc<Mutex<Publisher>>,
    store: Arc<Mutex<SessionStore>>,
) {
    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => {
                println!("Read 0 bytes, connection closed");
                break;
            }
            Ok(_bytes) => {
                let trim_msg = line.trim_end();
                if trim_msg.is_empty() {
                    continue;
                } else {
                    let mut pub_locked = publisher.lock().await;
                }
            }
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
                break;
            }
        }
    }
}

/*
 * TODO:
 * - Get all incoming messages and return them to ALL connected clients
 *      - To do this I need to split the stream
 * - Create a client with additional data like client name etc
 *
 */
