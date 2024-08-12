use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("192.168.1.69:4080").await?;

    // Send a message to the server
    stream.write_all(b"Hello, server!").await?;

    // Buffer to store the server's response
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;

    // Print the server's response
    println!("Received from server: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
