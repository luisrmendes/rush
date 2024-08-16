use telegram::get_telegram_task;
use tokio::io::{self};
use tokio::net::TcpStream;

pub enum Operation {
    GetIpv4,
    GetIpv6,
    WakeupDesktop,
}

pub mod state_machine {
    pub struct Node<S> {
        pub state: S,
    }

    pub struct Connecting;
    pub struct Connected;
    pub struct Disconnected;

    impl Node<Connecting> {
        pub fn new() -> Node<Connecting> {
            let follower = Node {
                state: Connecting {},
            };
            follower
        }
    }

    impl From<Node<Connecting>> for Node<Connected> {
        fn from(state: Node<Connecting>) -> Node<Connected> {
            let candidate = Node {
                state: Connected {},
            };
            candidate
        }
    }
}

pub mod commands {
    use crate::Operation;

    pub fn execute_commands(op: Operation) {
        match op {
            Operation::GetIpv4 => {}
            Operation::GetIpv6 => {}
            Operation::WakeupDesktop => {}
            _ => {}
        }
    }
}

pub mod telegram {
    use crate::Operation;

    fn parse_commands(text: Option<&str>) -> Option<Operation> {
        let parsed_msg: &str = match text {
            Some(msg) => msg,
            None => "",
        };
        match parsed_msg {
            "/ipv4" => Some(Operation::GetIpv4),
            "/ipv6" => Some(Operation::GetIpv6),
            "/desktop_wakeup" => Some(Operation::WakeupDesktop),
            _ => None,
        }
    }

    pub async fn get_telegram_task() {
        use teloxide::prelude::*;

        pretty_env_logger::init();
        log::info!("Starting throw dice bot...");

        let bot = Bot::from_env();

        return teloxide::repl(bot, |bot: Bot, msg: Message| async move {
            println!("{:?}", msg.text());

            let command = parse_commands(msg.text());
            bot.send_dice(msg.chat.id).await?;
            Ok(())
        })
        .await;
    }
}

pub mod office_env {
    use tokio::io::AsyncReadExt;
    use tokio::net::TcpStream;
    use tokio::task;

    #[derive(Debug)]
    pub struct OfficeEnv {
        brightness: u16,
        temperature: u16,
        humidity: u16,
    }

    /// Fetches Office Environment Data from the ESP8266 TCP Server
    pub fn get_office_env_task(
        mut stream: TcpStream,
    ) -> tokio::task::JoinHandle<Result<OfficeEnv, ()>> {
        return task::spawn(async move {
            let mut buffer = [0; 512];

            let recv_length: usize = match stream.read(&mut buffer).await {
                Ok(0) => {
                    println!("Connection closed by the server");
                    return Err(());
                }
                Ok(n) => {
                    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
                    n
                }
                Err(e) => {
                    eprintln!("Failed to read from socket; error = {:?}", e);
                    return Err(());
                }
            };

            let env_data: Result<Vec<u16>, _> = String::from_utf8_lossy(&buffer[..recv_length])
                .split_whitespace()
                .map(|s| s.parse::<u16>())
                .collect();

            let env_data: OfficeEnv = match env_data {
                Ok(env_data) => OfficeEnv {
                    brightness: env_data[1],
                    temperature: env_data[2],
                    humidity: env_data[3],
                },
                Err(e) => {
                    eprintln!("Failed parse info; error = {:?}", e);
                    return Err(());
                }
            };

            return Ok(env_data);
        });
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    use crate::state_machine::*;

    let follower = state_machine::Node {
        state: Connecting {},
    };
    let candidate = Node::<Connected>::from(follower);

    let stream = TcpStream::connect("192.168.1.69:4080").await?;
    println!("Connected to the server!");

    match office_env::get_office_env_task(stream).await {
        Ok(Ok(office_env)) => {
            println!("Final buffer received: {:?}", office_env);
        }
        Ok(Err(e)) => {
            eprintln!("Failed to read from socket: {:?}", e);
        }
        Err(e) => {
            eprintln!("Task failed: {:?}", e);
        }
    };

    get_telegram_task().await;

    Ok(())
}
