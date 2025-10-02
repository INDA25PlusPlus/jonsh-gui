use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn move_listener() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server is Listening on 127.0.0.1:7878");
    listener.set_nonblocking(true)?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("New connection: {}", stream.peer_addr()?);

        // Read data
        loop {
            let mut buffer = [0; 512];
            let bytes_read = match stream.read(&mut buffer) {
                Ok(0) => {
                    // connection closed by client
                    println!("Client disconnected");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error reading: {}", e);
                    break;
                }
            };
            println!(
                "Received: {}",
                String::from_utf8_lossy(&buffer[..bytes_read])
            );

            // Send response
            stream.write_all(b"Hello from server!")?;
        }
    }
    Ok(())
}
