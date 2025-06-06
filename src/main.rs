mod commands;
mod cygnus_ctrl_fsm;
mod get_ambient_fsm;
mod light_livingroom_ctrl_fsm;
mod llm_wrapper;
mod snowdog_ctrl_fsm;
mod telegram_bot;

use dotenv::dotenv;
use llm_wrapper::Llm;
use log::{debug, info, warn, LevelFilter};
use simplelog::{CombinedLogger, Config, WriteLogger};
use std::{fs::File, sync::Arc};
use telegram_bot::TelegramBot;
use tokio::{
    signal,
    sync::{broadcast, Mutex},
};

#[derive(Clone, Debug)]
pub struct GlobalState {
    am_i_home: bool,
    is_she_home: bool,
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

#[allow(dead_code)]
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

    let pc_name_array = ["SYRINX_VARS", "SNOWDOG_VARS", "CYGNUS_VARS", "RPI3_VARS"];

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
        LevelFilter::Debug,
        Config::default(),
        File::create("/tmp/rush.log").unwrap(),
    )])
    .unwrap();

    let global_state: Arc<Mutex<GlobalState>> = Arc::new(Mutex::new(GlobalState {
        am_i_home: false,
        is_she_home: false,
        office_env: OfficeAmbient {
            brightness: 0,
            temperature: 0,
            humidity: 0,
        },
    }));

    let systems = load_env_vars();

    match commands::check_external_system_connection(&systems.pcs).await {
        Ok(out) => {
            info!("{out}");
            println!("{out}");
        }
        Err(e) => {
            warn!("{e}");
            eprintln!("{e}");
        }
    };

    let (shutdown_tx, _) = broadcast::channel(1);

    // Ambient FSM
    let mut ambient_fsm = get_ambient_fsm::Fsm::new(systems.clone(), global_state.clone());
    let mut shutdown_rx = shutdown_tx.subscribe();
    let ambient_fsm_handle = tokio::spawn(async move {
        tokio::select! {
            () = ambient_fsm.run() => {},
            _ = shutdown_rx.recv() => {
                debug!("ambient_fsm received shutdown signal");
            }
        }
    });

    // Cygnus FSM
    let mut cygnus_fsm = cygnus_ctrl_fsm::Fsm::new(systems.pcs[2].clone(), global_state.clone());
    let mut shutdown_rx = shutdown_tx.subscribe();
    let cygnus_fsm_handle = tokio::spawn(async move {
        tokio::select! {
            () = cygnus_fsm.run() => {},
            _ = shutdown_rx.recv() => {
                debug!("cygnus_fsm received shutdown signal");
            }
        }
    });

    // Snowdog FSM
    let mut snowdog_fsm = snowdog_ctrl_fsm::Fsm::new(systems.pcs[1].clone(), global_state.clone());
    let mut shutdown_rx = shutdown_tx.subscribe();
    let snowdog_fsm_handle = tokio::spawn(async move {
        tokio::select! {
            () = snowdog_fsm.run() => {},
            _ = shutdown_rx.recv() => {
                debug!("snowdog_fsm received shutdown signal");
            }
        }
    });

    // Telegram Bot: REPL
    let llm_wrapper = Llm::new();
    let telegram_bot = TelegramBot::new(systems.clone(), global_state.clone(), llm_wrapper).await;
    let mut shutdown_rx = shutdown_tx.subscribe();
    let bot_clone = telegram_bot.clone();
    let telegram_bot_repl_handle = tokio::spawn(async move {
        tokio::select! {
            () = bot_clone.run_repl() => {},
            _ = shutdown_rx.recv() => {
                debug!("telegram_bot.answer_commands received shutdown signal");
            }
        }
    });

    // Telegram Bot: Update Am I Home
    let mut shutdown_rx = shutdown_tx.subscribe();
    let update_home_presence_handle = tokio::spawn(async move {
        tokio::select! {
            () = telegram_bot.update_home_presence() => {},
            _ = shutdown_rx.recv() => {
                debug!("telegram_bot.update_am_i_home received shutdown signal");
            }
        }
    });

    // Get Am I home
    let mut shutdown_rx = shutdown_tx.subscribe();
    let global_state_clone = global_state.clone();
    let get_home_presence_handle = tokio::spawn(async move {
        tokio::select! {
            () = commands::get_home_presence(global_state_clone) => {},
            _ = shutdown_rx.recv() => {
                debug!("get_am_i_home received shutdown signal");
            }
        }
    });

    let mut light_livingroom_fsm = light_livingroom_ctrl_fsm::Fsm::new(global_state.clone());
    let mut shutdown_rx = shutdown_tx.subscribe();
    let handle_light_livingroom_fsm = tokio::spawn(async move {
        tokio::select! {
            () = light_livingroom_fsm.run() => {},
            _ = shutdown_rx.recv() => {
                debug!("light_ctrl_fsm received shutdown signal");
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
        ambient_fsm_handle,
        cygnus_fsm_handle,
        snowdog_fsm_handle,
        telegram_bot_repl_handle,
        update_home_presence_handle,
        get_home_presence_handle,
        handle_light_livingroom_fsm,
        shutdown_listener
    );
}
