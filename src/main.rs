mod commands;
mod desktop_control_fsm;
mod get_env_fsm;
mod telegram_bot;

use desktop_control_fsm::Fsm as desktop_ctrl_fsm;
use dotenv::dotenv;
use get_env_fsm::Fsm as env_fsm;
use std::{collections::HashMap, sync::Arc};
use telegram_bot::TelegramBot;
use tokio::{
    signal,
    sync::{broadcast, Mutex},
};

#[derive(Clone)]
struct System {
    user: String,
    ip: String,
    mac: Option<String>,
}

#[derive(Clone, Debug)]
struct OfficeEnv {
    brightness: u32,
    temperature: u32,
    humidity: u32,
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
        ("SYSTEM0_USER", String::new()),
        ("SYSTEM0_IP_ADDR", String::new()),
        ("SYSTEM1_USER", String::new()),
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
                user: env_var_map
                    .get("SYSTEM0_USER")
                    .expect("Why is this empty?")
                    .to_string(),
                mac: None,
                ip: env_var_map
                    .get("SYSTEM0_IP_ADDR")
                    .expect("Why is this empty?")
                    .to_string(),
            },
            System {
                user: env_var_map
                    .get("SYSTEM1_USER")
                    .expect("Why is this empty?")
                    .to_string(),
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

    // TODO: Check if we have ssh access to all pcs

    println!("{}", commands::is_online(ctx.systems[1].clone()).await);

    // set some default data on office_env
    let office_env = OfficeEnv {
        brightness: 0,
        temperature: 0,
        humidity: 0,
    };
    let office_env = Arc::new(Mutex::new(office_env));

    let mut env_fsm = env_fsm::new(ctx.clone(), office_env.clone());
    let mut desktop_ctrl_fsm = desktop_ctrl_fsm::new(ctx.systems[1].clone(), office_env.clone());
    let telegram_bot = TelegramBot::new(ctx.clone());

    // Create a broadcast channel for shutdown signal
    let (shutdown_tx, _) = broadcast::channel(1);

    // Task 1: Office environment FSM
    let mut shutdown_rx1 = shutdown_tx.subscribe(); // Subscribe to the shutdown signal
    let handle1 = tokio::spawn(async move {
        tokio::select! {
            _ = env_fsm.run() => {},
            _ = shutdown_rx1.recv() => {
                println!("env_fsm received shutdown signal");
            }
        }
    });

    // Task 2: Desktop control FSM
    let mut shutdown_rx2 = shutdown_tx.subscribe(); // Subscribe to the shutdown signal
    let handle2 = tokio::spawn(async move {
        tokio::select! {
            _ = desktop_ctrl_fsm.run() => {},
            _ = shutdown_rx2.recv() => {
                println!("desktop_ctrl_fsm received shutdown signal");
            }
        }
    });

    // Task 3: Telegram bot
    let mut shutdown_rx3 = shutdown_tx.subscribe(); // Subscribe to the shutdown signal
    let handle3 = tokio::spawn(async move {
        tokio::select! {
            _ = telegram_bot.run() => {},
            _ = shutdown_rx3.recv() => {
                println!("telegram_bot received shutdown signal");
            }
        }
    });

    // Task 4: Listen for Ctrl-C and broadcast the shutdown signal
    let shutdown_listener = tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl-C");
        println!("Ctrl-C received, sending shutdown signal...");
        let _ = shutdown_tx.send(()); // Broadcast the shutdown signal
    });

    // Await all tasks to complete
    let _ = tokio::try_join!(handle1, handle2, handle3, shutdown_listener);
}
