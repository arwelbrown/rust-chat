use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> Result<(), io::Error> {
    println!("Opening a TCP connection...");

    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream)?,
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<(), io::Error> {
    let mut buf = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            break;
        }

        let msg = String::from_utf8_lossy(&buf[..bytes_read]);
        println!("Received: {}", msg);
        let response = format!("> {}", msg);

        stream.write_all(response.as_bytes())?;
    }

    Ok(())
}
