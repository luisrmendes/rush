use std::sync::Arc;

use log::debug;
use log::warn;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio::time::timeout;
use tokio::time::Duration;

use crate::Context;
use crate::GlobalState;
use crate::OfficeEnv;
enum State {
    Connecting,
    Connected,
}

pub struct Fsm {
    state: State,
    context: Context,
    global_state: Arc<Mutex<GlobalState>>,
    stream: Option<TcpStream>,
}

impl Fsm {
    pub fn new(ctx: Context, global_state: Arc<Mutex<GlobalState>>) -> Self {
        Self {
            state: State::Connecting,
            context: ctx,
            global_state,
            stream: None,
        }
    }

    pub async fn run(&mut self) {
        loop {
            match self.state {
                State::Connecting => self.connecting().await,
                State::Connected => self.connected().await,
            }
        }
    }

    async fn connecting(&mut self) {
        self.stream = None;
        let address = &self.context.env_sensor_address_port;
        debug!("Attempting to connect to {address}");

        match TcpStream::connect(address).await {
            Ok(stream) => {
                debug!("Successfully connected!");
                self.stream = Some(stream);
                self.state = State::Connected;
            }
            Err(e) => {
                debug!("Failed to connect: {e}");
                sleep(Duration::from_secs(2)).await;
            }
        }
    }

    async fn connected(&mut self) {
        use tokio::io::AsyncReadExt;

        let Some(stream) = &mut self.stream else {
            debug!("Tcp stream is unavailable");
            self.state = State::Connecting;
            return;
        };

        let mut buffer = [0; 4096];

        let recv_length: usize =
            match timeout(Duration::from_secs(2), stream.read(&mut buffer)).await {
                Ok(Ok(0)) => {
                    warn!("Connection gracefully closed");
                    self.state = State::Connecting;
                    return;
                }
                Ok(Ok(n)) => {
                    debug!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
                    n
                }
                Ok(Err(e)) => {
                    warn!("Read error: {e:?}. Going to Disconnected");
                    self.state = State::Connecting;
                    return;
                }
                Err(_) => {
                    warn!("Timed out waiting for data");
                    self.state = State::Connecting;
                    return;
                }
            };

        let office_env: Result<Vec<u32>, _> = String::from_utf8_lossy(&buffer[..recv_length])
            .split_whitespace()
            .map(str::parse)
            .collect();

        let office_env: OfficeEnv = match office_env {
            Ok(office_env) => OfficeEnv {
                brightness: office_env[1],
                temperature: office_env[2],
                humidity: office_env[3],
            },
            Err(e) => {
                warn!("Failed parse info. Error = {e:?}");
                return;
            }
        };

        let mut global_state = self.global_state.lock().await;
        global_state.office_env = office_env;
    }
}
