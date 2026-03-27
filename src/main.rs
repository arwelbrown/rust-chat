use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> Result<(), io::Error> {
    let socket = TcpListener::bind("127.0.0.1:7878")?;

    println!("Server listening on port 7878...");

    for stream in socket.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from: {}", stream.peer_addr()?);
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
    let client_name = stream.peer_addr()?.to_string();
    let mut buf = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buf)?;

        if bytes_read == 0 {
            break;
        }

        let msg = String::from_utf8_lossy(&buf[..bytes_read]);
        println!("Received: {} from client: {}", msg, client_name);
        let response = format!("> {}", msg);

        stream.write_all(response.as_bytes())?;
    }

    Ok(())
}

/*
 * TODO:
 * - Get all incoming messages and return them to ALL connected clients
 *      - To do this I need to split the stream
 * - Create a client with additional data like client name etc
 *
 */
