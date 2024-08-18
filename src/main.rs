mod commands;
mod office_env_fsm;
mod telegram_bot;
use dotenv::dotenv;
use office_env_fsm::Fsm;
use std::collections::HashMap;
use telegram_bot::TelegramBot;
use tokio::{
    signal,
    sync::{broadcast, oneshot},
};

#[derive(Clone)]
struct System {
    ip: String,
    mac: Option<String>,
}

#[derive(Clone)]
pub struct Context {
    env_sensor_address_port: String,
    systems: Vec<System>,
}

fn load_env_vars() -> Context {
    dotenv().ok();

    // Check for the expected env vars
    let mut env_var_map = HashMap::from([
        ("ESP8266_ADDRESS_PORT", String::new()),
        ("SYSTEM0_IP_ADDR", String::new()),
        ("SYSTEM1_IP_ADDR", String::new()),
        ("SYSTEM1_MAC", String::new()),
        ("SYSTEM2_IP_ADDR", String::new()),
    ]);

    for (env_var, value) in env_var_map.iter_mut() {
        let val = std::env::var(env_var).expect(&format!("{env_var} must be set."));
        if val.is_empty() {
            panic!("{env_var} is empty. Please set it.");
        }
        *value = val;
    }

    Context {
        env_sensor_address_port: env_var_map
            .get("ESP8266_ADDRESS_PORT")
            .expect("Why is this empty?")
            .to_string(),
        systems: vec![
            System {
                mac: None,
                ip: env_var_map
                    .get("SYSTEM0_IP_ADDR")
                    .expect("Why is this empty?")
                    .to_string(),
            },
            System {
                mac: Some(
                    env_var_map
                        .get("SYSTEM1_MAC")
                        .expect("Why is this empty?")
                        .to_string(),
                ),
                ip: env_var_map
                    .get("SYSTEM1_IP_ADDR")
                    .expect("Why is this empty?")
                    .to_string(),
            },
        ],
    }

    //println!("Production Url: {}", &esp8266_address);
}

#[tokio::main]
async fn main() {
    let ctx = load_env_vars();

    let mut office_env_fsm = Fsm::new(ctx.clone());
    let telegram_bot = TelegramBot::new(ctx.clone());

    // Create a broadcast channel for shutdown signal
    let (shutdown_tx, _) = broadcast::channel(1);

    // Task 1: Office environment FSM
    let mut shutdown_rx1 = shutdown_tx.subscribe(); // Subscribe to the shutdown signal
    let handle1 = tokio::spawn(async move {
        tokio::select! {
            _ = office_env_fsm.run() => {},
            _ = shutdown_rx1.recv() => {
                println!("office_env_fsm received shutdown signal");
            }
        }
    });

    // Task 2: Telegram bot
    let mut shutdown_rx2 = shutdown_tx.subscribe(); // Subscribe to the shutdown signal
    let handle2 = tokio::spawn(async move {
        tokio::select! {
            _ = telegram_bot.run() => {},
            _ = shutdown_rx2.recv() => {
                println!("telegram_bot received shutdown signal");
            }
        }
    });

    // Task 3: Listen for Ctrl-C and broadcast the shutdown signal
    let shutdown_listener = tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl-C");
        println!("Ctrl-C received, sending shutdown signal...");
        let _ = shutdown_tx.send(()); // Broadcast the shutdown signal
    });

    // Await all tasks to complete
    let _ = tokio::try_join!(handle1, handle2, shutdown_listener);
}
