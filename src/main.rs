use std::future::Future;

use tokio::io::{self, AsyncReadExt};
use tokio::net::TcpStream;
use tokio::task;

#[derive(Debug)]
pub struct OfficeEnv {
    brightness: u16,
    temperature: u16,
    humidity: u16,
}

async fn get_telegram_task()  {
    use teloxide::prelude::*;

    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    return teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    }).await;
}

/// Fetches Office Environment Data from the ESP8266 TCP Server
fn get_office_env_task(mut stream: TcpStream) -> tokio::task::JoinHandle<Result<OfficeEnv, ()>> {
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

        let result: Result<Vec<u16>, _> = String::from_utf8_lossy(&buffer[..recv_length])
            .split_whitespace()
            .map(|s| s.parse::<u16>())
            .collect();

        let env_data: OfficeEnv = match result {
            Ok(result) => OfficeEnv {
                brightness: result[1],
                temperature: result[2],
                humidity: result[3],
            },
            Err(e) => {
                eprintln!("Failed parse info; error = {:?}", e);
                return Err(());
            }
        };

        return Ok(env_data);
    });
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let stream = TcpStream::connect("192.168.1.69:4080").await?;
    println!("Connected to the server!");

    match get_office_env_task(stream).await {
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
