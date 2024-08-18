use tokio::io;
use tokio::io::Interest;
use tokio::net::TcpStream;
use tokio::time::sleep;
use tokio::time::timeout;
use tokio::time::Duration;
enum State {
    Connecting,
    Connected,
    Disconnected,
}

pub struct Fsm {
    state: State,
    stream: Option<TcpStream>,
}

#[derive(Debug)]
struct OfficeEnv {
    brightness: u16,
    temperature: u16,
    humidity: u16,
}

impl Fsm {
    pub fn new() -> Self {
        Self {
            state: State::Disconnected,
            stream: None,
        }
    }

    pub async fn run(&mut self) {
        loop {
            match self.state {
                State::Connecting => self.connect().await,
                State::Connected => self.connected().await,
                State::Disconnected => self.disconnected().await,
            }
        }
    }

    async fn connect(&mut self) {
        let address = "192.168.1.99:4080";
        println!("Attempting to connect to {}", address);
        
        match TcpStream::connect(address).await {
            Ok(stream) => {
                println!("Successfully connected!");
                self.stream = Some(stream);
                self.state = State::Connected;
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
                self.state = State::Disconnected
            }
        }
    }

    async fn connected(&mut self) {
        use tokio::io::AsyncReadExt;

        let stream = match &mut self.stream {
            Some(stream) => {
                stream
            }
            None => {
                println!("Failed to connect");
                self.state = State::Disconnected;
                return;
            }
        };

        let mut buffer = [0; 4096];

        let recv_length: usize =
            match timeout(Duration::from_secs(2), stream.read(&mut buffer)).await {
                Ok(Ok(0)) => {
                    println!("Connection gracefully closed");
                    self.state = State::Disconnected;
                    return;
                }
                Ok(Ok(n)) => {
                    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
                    n
                }
                Ok(Err(e)) => {
                    eprintln!("Read error: {:?}", e);
                    self.state = State::Disconnected;
                    return;
                }
                Err(_) => {
                    println!("Timed out waiting for data");
                    self.state = State::Disconnected;
                    return;
                }
            };

        let env_data: Result<Vec<u16>, _> = String::from_utf8_lossy(&buffer[..recv_length])
            .split_whitespace()
            .map(|s| s.parse::<u16>())
            .collect();

        let _env_data: OfficeEnv = match env_data {
            Ok(env_data) => OfficeEnv {
                brightness: env_data[1],
                temperature: env_data[2],
                humidity: env_data[3],
            },
            Err(e) => {
                eprintln!("Failed parse info; error = {:?}", e);
                return;
            }
        };

        // TODO: Update data repositories
    }

    async fn disconnected(&mut self) {
        println!("Disconnected");
        // TODO: Set some state in the data repositories that makes sense
        self.state = State::Connecting;
        self.stream = None;
    }
}
