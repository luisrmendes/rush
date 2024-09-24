mod commands;
mod get_ambient_fsm;
mod llm_wrapper;
mod snowdog_ctrl_fsm;
mod telegram_bot;
mod thinkpad_ctrl_fsm;
mod tui;

use dotenv::dotenv;
use get_ambient_fsm::Fsm as ambient_fsm;
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
    office_env: OfficeAmbient,
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
struct OfficeAmbient {
    brightness: u32,
    temperature: u32,
    humidity: u32,
}

fn load_env_vars() -> Systems {
    dotenv().ok();

    let mut pcs = vec![];

    let env_var = "TELOXIDE_TOKEN";
    std::env::var(env_var).unwrap_or_else(|_| panic!("{env_var} must be set."));

    let env_var = "ESP8266_ADDRESS_PORT";
    let var_content = std::env::var(env_var).unwrap_or_else(|_| panic!("{env_var} must be set."));

    let mut hostname: String = String::new();
    let mut port: String = String::new();

    for part in var_content.split(';') {
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
    let esp8266_rush = Embedded { hostname, port };

    let pc_name_array = [
        "SYRINX_VARS",
        "SNOWDOG_VARS",
        "THINKPADX1_VARS",
        "RPI5_VARS",
        "RPI3_VARS",
    ];

    // Filter PC env vars
    for pc_name in pc_name_array {
        let var_content =
            std::env::var(pc_name).unwrap_or_else(|_| panic!("{pc_name} must be set."));

        let mut user = String::new();
        let mut ip = String::new();
        let mut mac: Option<String> = None;

        for part in var_content.split(';') {
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
                "mac" => mac = Some(value.to_string()),
                other => {
                    panic!("Not handling key {other}")
                }
            }
        }

        pcs.push(Pc { user, ip, mac });
    }

    Systems { esp8266_rush, pcs }
}

#[allow(clippy::too_many_lines)]
#[tokio::main]
async fn main() {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create("/tmp/rush.log").unwrap(),
    )])
    .unwrap();

    let global_state: Arc<Mutex<GlobalState>> = Arc::new(Mutex::new(GlobalState {
        am_i_home: false,
        office_env: OfficeAmbient {
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

    let mut ambient_fsm = ambient_fsm::new(systems.clone(), global_state.clone());
    let mut thinkpad_fsm = thinkpad_fsm::new(systems.pcs[2].clone(), global_state.clone());
    let mut snowdog_fsm = snowdog_fsm::new(systems.pcs[1].clone(), global_state.clone());
    let llm_wrapper = Llm::new("http://localhost:11434/api/generate", "llama3.1");
    let telegram_bot = TelegramBot::new(systems.clone(), global_state.clone(), llm_wrapper).await;
    let tui = Tui::new(global_state.clone(), systems);

    let (shutdown_tx, _) = broadcast::channel(1);

    // Office ambient FSM
    let mut shutdown_rx = shutdown_tx.subscribe();
    let handle1 = tokio::spawn(async move {
        tokio::select! {
            () = ambient_fsm.run() => {},
            _ = shutdown_rx.recv() => {
                debug!("env_fsm received shutdown signal");
            }
        }
    });

    // Desktop control FSM
    let mut shutdown_rx = shutdown_tx.subscribe();
    let handle2 = tokio::spawn(async move {
        tokio::select! {
            () = thinkpad_fsm.run() => {},
            _ = shutdown_rx.recv() => {
                debug!("thinkpad_fsm received shutdown signal");
            }
        }
    });

    // Telegram bot answer commands
    let mut shutdown_rx = shutdown_tx.subscribe();
    let bot_clone = telegram_bot.clone();
    let handle3 = tokio::spawn(async move {
        tokio::select! {
            () = bot_clone.run_repl() => {},
            _ = shutdown_rx.recv() => {
                debug!("telegram_bot.answer_commands received shutdown signal");
            }
        }
    });

    // Get Am I home
    let mut shutdown_rx = shutdown_tx.subscribe();
    let handle4 = tokio::spawn(async move {
        tokio::select! {
            () = commands::get_am_i_home(global_state.clone()) => {},
            _ = shutdown_rx.recv() => {
                debug!("get_am_i_home received shutdown signal");
            }
        }
    });

    // Telegram Bot Update Am I Home
    let mut shutdown_rx = shutdown_tx.subscribe();
    let handle5 = tokio::spawn(async move {
        tokio::select! {
            () = telegram_bot.update_am_i_home() => {},
            _ = shutdown_rx.recv() => {
                debug!("telegram_bot.update_am_i_home received shutdown signal");
            }
        }
    });

    // TUI
    let mut shutdown_rx = shutdown_tx.subscribe();
    let handle6 = tokio::spawn(async move {
        tokio::select! {
            _ = tui.run() => {},
            _ = shutdown_rx.recv() => {
                debug!("tui received shutdown signal");
            }
        }
    });

    // Snowdog FSM
    let mut shutdown_rx = shutdown_tx.subscribe();
    let handle7 = tokio::spawn(async move {
        tokio::select! {
            () = snowdog_fsm.run() => {},
            _ = shutdown_rx.recv() => {
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
