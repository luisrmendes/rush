mod commands;
mod get_env_fsm;
mod llm_wrapper;
mod snowdog_ctrl_fsm;
mod telegram_bot;
mod thinkpad_ctrl_fsm;
mod tui;

use dotenv::dotenv;
use get_env_fsm::Fsm as env_fsm;
use llm_wrapper::Llm;
use log::{debug, info, warn, LevelFilter};
use simplelog::{CombinedLogger, Config, WriteLogger};
use snowdog_ctrl_fsm::Fsm as snowdog_fsm;
use std::{fs::File, sync::Arc};
use telegram_bot::TelegramBot;
use thinkpad_ctrl_fsm::Fsm as thinkpad_fsm;
use tokio::{
    signal,
    sync::{broadcast, Mutex},
};
use tui::Tui;

#[derive(Clone, Debug)]
pub struct GlobalState {
    am_i_home: bool,
    office_env: OfficeEnv,
}

#[derive(Clone, Debug)]
pub struct Systems {
    esp8266_rush: Embedded,
    pcs: Vec<Pc>,
}

#[derive(Clone, Debug)]
struct Pc {
    user: String,
    ip: String,
    mac: Option<String>,
}

#[derive(Clone, Debug)]
struct Embedded {
    hostname: String,
    port: String,
}

#[derive(Clone, Debug)]
struct OfficeEnv {
    brightness: u32,
    temperature: u32,
    humidity: u32,
}

//TODO: this needs a refactor
fn load_env_vars() -> Systems {
    dotenv().ok();

    std::env::var("TELOXIDE_TOKEN").unwrap_or_else(|_| panic!("TELOXIDE_TOKEN must be set."));

    // Check for the expected env vars
    let env_var_map = vec![
        "ESP8266_ADDRESS_PORT",
        "SYRINX_VARS",
        "SNOWDOG_VARS",
        "THINKPADX1_VARS",
        "RPI5_VARS",
        "RPI3_VARS",
    ];

    let mut pcs = vec![];
    let mut esp8266_rush = Embedded {
        hostname: String::new(),
        port: String::new(),
    };

    for env_var in env_var_map {
        let system_vars =
            std::env::var(env_var).unwrap_or_else(|_| panic!("{env_var} must be set."));
        assert!(
            !system_vars.is_empty(),
            "{env_var} is empty. Please set it."
        );

        // Parse the SYSTEMX_VARS string
        match env_var {
            "ESP8266_ADDRESS_PORT" => {
                let val =
                    std::env::var(env_var).unwrap_or_else(|_| panic!("{env_var} must be set."));
                assert!(!val.is_empty(), "{env_var} is empty. Please set it.");

                let mut hostname: String = String::new();
                let mut port: String = String::new();

                for part in system_vars.split(';') {
                    let mut key_value = part.split('=');
                    let Some(key) = key_value.next() else {
                        panic!("{part} has no next!")
                    };
                    let Some(value) = key_value.next() else {
                        panic!("{part} has no next!")
                    };

                    match key {
                        "hostname" => hostname = value.to_string(),
                        "port" => port = value.to_string(),
                        other => {
                            panic!("Not handling key {other}")
                        }
                    }
                }
                esp8266_rush = Embedded { hostname, port };
            }
            "SNOWDOG_VARS" => {
                let mut user = String::new();
                let mut ip = String::new();
                let mut mac = String::new();

                for part in system_vars.split(';') {
                    let mut key_value = part.split('=');
                    let Some(key) = key_value.next() else {
                        panic!("{part} has no next!")
                    };
                    let Some(value) = key_value.next() else {
                        panic!("{part} has no next!")
                    };

                    match key {
                        "user" => user = value.to_string(),
                        "ip" => ip = value.to_string(),
                        "mac" => mac = value.to_string(),
                        other => {
                            panic!("Not handling key {other}")
                        }
                    }
                }
                pcs.push(Pc {
                    user,
                    ip,
                    mac: Some(mac),
                });
            }
            "SYRINX_VARS" | "THINKPADX1_VARS" | "RPI5_VARS" | "RPI3_VARS" => {
                let mut user = String::new();
                let mut ip = String::new();

                for part in system_vars.split(';') {
                    let mut key_value = part.split('=');
                    let Some(key) = key_value.next() else {
                        panic!("{part} has no next!")
                    };
                    let Some(value) = key_value.next() else {
                        panic!("{part} has no next!")
                    };

                    match key {
                        "user" => user = value.to_string(),
                        "ip" => ip = value.to_string(),
                        other => {
                            panic!("Not handling key {other}")
                        }
                    }
                }
                pcs.push(Pc {
                    user,
                    ip,
                    mac: None,
                });
            }
            other => {
                panic!("Not handling env var {other}")
            }
        }
    }

    Systems { esp8266_rush, pcs }
}

#[tokio::main]
async fn main() {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        File::create("/tmp/rush.log").unwrap(),
    )])
    .unwrap();

    let global_state: Arc<Mutex<GlobalState>> = Arc::new(Mutex::new(GlobalState {
        am_i_home: false,
        office_env: OfficeEnv {
            brightness: 0,
            temperature: 0,
            humidity: 0,
        },
    }));

    let systems = load_env_vars();

    match commands::check_external_system_connection(&systems.pcs).await {
        Ok(out) => info!("{}", out),
        Err(e) => warn!("{}", e),
    };

    let mut env_fsm = env_fsm::new(systems.clone(), global_state.clone());
    let mut thinkpad_fsm = thinkpad_fsm::new(systems.pcs[2].clone(), global_state.clone());
    let mut snowdog_fsm = snowdog_fsm::new(systems.pcs[1].clone(), global_state.clone());
    let llm_wrapper = Llm::new("http://localhost:11434/api/generate", "llama3.1");
    let telegram_bot = TelegramBot::new(systems.clone(), global_state.clone(), llm_wrapper).await;
    let tui = Tui::new(global_state.clone(), systems);

    // TODO: Simplify the task spawning

    // Create a broadcast channel for shutdown signal
    let (shutdown_tx, _) = broadcast::channel(1);

    // Task 1: Office environment FSM
    let mut shutdown_rx1 = shutdown_tx.subscribe(); // Subscribe to the shutdown signal
    let handle1 = tokio::spawn(async move {
        tokio::select! {
            () = env_fsm.run() => {},
            _ = shutdown_rx1.recv() => {
                debug!("env_fsm received shutdown signal");
            }
        }
    });

    // Task 2: Desktop control FSM
    let mut shutdown_rx2 = shutdown_tx.subscribe(); // Subscribe to the shutdown signal
    let handle2 = tokio::spawn(async move {
        tokio::select! {
            () = thinkpad_fsm.run() => {},
            _ = shutdown_rx2.recv() => {
                debug!("thinkpad_fsm received shutdown signal");
            }
        }
    });

    // Task 3: Telegram bot answer commands
    let mut shutdown_rx3 = shutdown_tx.subscribe(); // Subscribe to the shutdown signal
    let bot_clone = telegram_bot.clone();
    let handle3 = tokio::spawn(async move {
        tokio::select! {
            () = bot_clone.run_repl() => {},
            _ = shutdown_rx3.recv() => {
                debug!("telegram_bot.answer_commands received shutdown signal");
            }
        }
    });

    // Task 4: Get Am I home
    let mut shutdown_rx4 = shutdown_tx.subscribe(); // Subscribe to the shutdown signal
    let handle4 = tokio::spawn(async move {
        tokio::select! {
            () = commands::get_am_i_home(global_state.clone()) => {},
            _ = shutdown_rx4.recv() => {
                debug!("get_am_i_home received shutdown signal");
            }
        }
    });

    // Task 5: Telegram Bot Update Am I Home
    let mut shutdown_rx5 = shutdown_tx.subscribe(); // Subscribe to the shutdown signal
    let handle5 = tokio::spawn(async move {
        tokio::select! {
            () = telegram_bot.update_am_i_home() => {},
            _ = shutdown_rx5.recv() => {
                debug!("telegram_bot.update_am_i_home received shutdown signal");
            }
        }
    });

    // Task 6: TUI
    let mut shutdown_rx6 = shutdown_tx.subscribe(); // Subscribe to the shutdown signal
    let handle6 = tokio::spawn(async move {
        tokio::select! {
            _ = tui.run() => {},
            _ = shutdown_rx6.recv() => {
                debug!("tui received shutdown signal");
            }
        }
    });

    // Task 7: Snowdog FSM
    let mut shutdown_rx7 = shutdown_tx.subscribe(); // Subscribe to the shutdown signal
    let handle7 = tokio::spawn(async move {
        tokio::select! {
            () = snowdog_fsm.run() => {},
            _ = shutdown_rx7.recv() => {
                debug!("snowdog_fsm received shutdown signal");
            }
        }
    });

    // Listen for Ctrl-C and broadcast the shutdown signal
    let shutdown_listener = tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl-C");
        debug!("Ctrl-C received, sending shutdown signal...");
        let _ = shutdown_tx.send(()); // Broadcast the shutdown signal
    });

    // Await all tasks to complete
    let _ = tokio::try_join!(
        handle1,
        handle2,
        handle3,
        handle4,
        handle5,
        handle6,
        handle7,
        shutdown_listener
    );
}
