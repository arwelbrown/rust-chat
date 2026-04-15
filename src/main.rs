pub mod models;
pub mod utils;
use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::Arc,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, tcp::OwnedReadHalf, tcp::OwnedWriteHalf},
    sync::{
        Mutex,
        mpsc::{self},
    },
};
use uuid::Uuid;

use crate::{
    models::{
        message::Message,
        publisher::Publisher,
        session::{Session, SessionStore},
        sqlite::SqLite,
        subscriber::Subscriber,
    },
    utils::utils::Utils,
};

const PORT: &str = "7878";

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT)).await?;
    let sessions = SessionStore::new();
    let store = Arc::new(Mutex::new(sessions));
    let conversations = HashMap::new();
    let db = SqLite::init("db".to_string())?;
    let publisher = Arc::new(Mutex::new(Publisher { conversations, db }));

    println!("Server listening on port {}...", PORT);

    loop {
        let (tcp_stream, socket_addr) = match listener.accept().await {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
                continue;
            }
        };

        println!("New connection from: {}", socket_addr);

        let (read, mut write) = tcp_stream.into_split();
        let mut reader = BufReader::new(read);

        // --- Handshake ---
        let mut first_line = String::new();
        let n = match reader.read_line(&mut first_line).await {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read handshake from {}: {}", socket_addr, e);
                continue;
            }
        };

        if n == 0 {
            println!("Connection closed during handshake: {}", socket_addr);
            continue;
        }

        let msg = first_line.trim_end().to_string();

        let (conversation_id, subscriber_id, _msg) = match Utils::formatter(&msg, publisher.clone())
        {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Failed to parse handshake from {}: {}", socket_addr, e);
                continue;
            }
        };

        // --- Session setup ---
        let (tx, rx) = mpsc::channel::<String>(100);
        {
            let mut sessions = store.lock().await;
            sessions.insert(
                subscriber_id,
                Session {
                    subscriber: Subscriber { id: subscriber_id },
                    tx: tx.clone(),
                },
            );
        }

        // --- Acknowledge ---
        acknowledge_handshake(
            subscriber_id,
            conversation_id,
            publisher.clone(),
            socket_addr,
            &mut write,
        )
        .await;

        // --- Spawn reader ---
        let store_clone = Arc::clone(&store);
        let publisher_clone = Arc::clone(&publisher);

        tokio::spawn(async move {
            if let Err(e) = reader_loop(
                reader,
                subscriber_id,
                conversation_id,
                publisher_clone,
                store_clone.clone(),
            )
            .await
            {
                eprintln!("Reader loop failed for {}: {}", subscriber_id, e);
            }

            store_clone.lock().await.remove(&subscriber_id);
            println!("Session {} removed", subscriber_id);
        });

        // --- Spawn writer ---
        let store_clone = Arc::clone(&store);
        tokio::spawn(writer_loop(
            rx,
            subscriber_id,
            store_clone,
            conversation_id,
            publisher.clone(),
            write,
        ));
    }
}

async fn acknowledge_handshake(
    subscriber_id: Uuid,
    conversation_id: Uuid,
    publisher: Arc<Mutex<Publisher>>,
    socket_addr: SocketAddr,
    write: &mut OwnedWriteHalf,
) {
    let ack = format!("ACK:{}", subscriber_id);
    if let Err(e) = write.write_all(ack.as_bytes()).await {
        eprintln!(
            "Failed to send handshake acknowledgement to {}: {}",
            socket_addr, e
        );
    }

    if let Err(e) = write.flush().await {
        eprintln!(
            "Failed to flush handshake acknowledgement to {}: {}",
            socket_addr, e
        );
    }

    println!("Sent ACK to client {}", subscriber_id);

    {
        let mut pub_locked = publisher.lock().await;
        let mut subs = HashSet::new();
        subs.insert(Subscriber { id: subscriber_id });
        pub_locked.sub_to_room(conversation_id, subs);
    }
}

async fn reader_loop(
    mut reader: BufReader<OwnedReadHalf>,
    subscriber_id: Uuid,
    conversation_id: Uuid,
    publisher: Arc<Mutex<Publisher>>,
    store: Arc<Mutex<SessionStore>>,
) -> Result<()> {
    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => {
                println!(
                    "Read 0 bytes from client {}, connection closed",
                    subscriber_id
                );
                break;
            }
            Ok(_bytes) => {
                let trim_msg = line.trim_end();
                if trim_msg.is_empty() {
                    continue;
                } else {
                    let mut pub_locked = publisher.lock().await;
                    pub_locked
                        .dispatch_messages(
                            conversation_id,
                            &Message::new(trim_msg.to_string()),
                            store.clone(),
                        )
                        .await;
                }
            }
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
                if e.kind() == std::io::ErrorKind::ConnectionReset
                    || e.kind() == std::io::ErrorKind::ConnectionAborted
                {
                    return Err(e.into());
                }

                continue;
            }
        }
    }

    Ok(())
}

async fn writer_loop(
    mut rx: mpsc::Receiver<String>,
    subscriber_id: Uuid,
    store: Arc<Mutex<SessionStore>>,
    conversation_id: Uuid,
    publisher: Arc<Mutex<Publisher>>,
    mut write: OwnedWriteHalf,
) {
    while let Some(msg) = rx.recv().await {
        println!("Broadcasting message from {}: {}", subscriber_id, msg);
        let mut pub_locked = publisher.lock().await;
        let subscriber = Subscriber { id: subscriber_id };
        let mut subs: HashSet<Subscriber> = HashSet::new();

        subs.insert(subscriber);
        pub_locked.sub_to_room(conversation_id, subs.to_owned());
        pub_locked.list_rooms();
        pub_locked
            .dispatch_messages(conversation_id, &Message::new(msg.clone()), store.clone())
            .await;

        let msg_with_newline = if msg.ends_with('\n') {
            msg
        } else {
            format!("{}\n", msg)
        };

        if let Err(e) = write.write_all(msg_with_newline.as_bytes()).await {
            eprintln!("Failed to send message to client {}: {}", subscriber_id, e);
            break;
        }

        if let Err(e) = write.flush().await {
            eprintln!(
                "Failed to flush message for client {}: {}",
                subscriber_id, e
            );
            break;
        }
        println!(
            "Sent message to client {}: {}",
            subscriber_id,
            msg_with_newline.trim_end()
        );
    }

    let mut store = store.lock().await;
    store.remove(&subscriber_id);
    println!("Session {} removed", subscriber_id);
}

// TODO:
// - are there any places where a Mutex can be replaced with an RwLock?
