use std::sync::Arc;

use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio::time::timeout;
use tokio::time::Duration;

use crate::Context;
use crate::OfficeEnv;
enum State {
    Connecting,
    Connected,
    Disconnected,
}

pub struct Fsm {
    state: State,
    context: Context,
    env_data: Arc<Mutex<OfficeEnv>>,
    stream: Option<TcpStream>,
}

impl Fsm {
    pub fn new(ctx: Context, env_data: Arc<Mutex<OfficeEnv>>) -> Self {
        Self {
            state: State::Disconnected,
            context: ctx,
            env_data: env_data,
            stream: None,
        }
    }

    pub async fn run(&mut self) {
        loop {
            match self.state {
                State::Connecting => self.connecting().await,
                State::Connected => self.connected().await,
                State::Disconnected => self.disconnected(),
            }
        }
    }

    async fn connecting(&mut self) {
        let address = &self.context.env_sensor_address_port;
        println!("Attempting to connect to {address}");

        match TcpStream::connect(address).await {
            Ok(stream) => {
                println!("Successfully connected!");
                self.stream = Some(stream);
                self.state = State::Connected;
            }
            Err(e) => {
                println!("Failed to connect: {e}");
                sleep(Duration::from_secs(2)).await;
            }
        }
    }

    async fn connected(&mut self) {
        use tokio::io::AsyncReadExt;

        let Some(stream) = &mut self.stream else {
            println!("Tcp stream is unavailable");
            self.state = State::Disconnected;
            return;
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
                    // TODO: log into pretty logs
                    //println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
                    n
                }
                Ok(Err(e)) => {
                    eprintln!("Read error: {e:?}");
                    self.state = State::Disconnected;
                    return;
                }
                Err(_) => {
                    println!("Timed out waiting for data");
                    self.state = State::Disconnected;
                    return;
                }
            };

        let env_data: Result<Vec<u32>, _> = String::from_utf8_lossy(&buffer[..recv_length])
            .split_whitespace()
            .map(str::parse)
            .collect();

        let env_data: OfficeEnv = match env_data {
            Ok(env_data) => OfficeEnv {
                brightness: env_data[1],
                temperature: env_data[2],
                humidity: env_data[3],
            },
            Err(e) => {
                eprintln!("Failed parse info; error = {e:?}");
                return;
            }
        };

        let mut stored_env_data = self.env_data.lock().await;
        *stored_env_data = env_data;
    }

    fn disconnected(&mut self) {
        println!("Disconnected");
        // TODO: Set some state in the data repositories that makes sense
        self.state = State::Connecting;
        self.stream = None;
    }
}
