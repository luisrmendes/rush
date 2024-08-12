use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Connect to the server at the specified address and port
    let mut stream = TcpStream::connect("192.168.1.69:4080").await?;
    println!("Connected to the server!");

    let mut buffer = [0; 1024];  // Buffer to store incoming data

    loop {
        // Read data from the stream
        let n = match stream.read(&mut buffer).await {
            Ok(n) if n == 0 => {
                // Connection was closed by the server
                println!("Connection closed by the server");
                break;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from socket; error = {:?}", e);
                break;
            }
        };

        // Print the received message
        println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
    }

    Ok(())
}
