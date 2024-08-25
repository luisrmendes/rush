mod commands;
mod get_env_fsm;
mod telegram_bot;
mod thinkpad_dock_control_fsm;

use dotenv::dotenv;
use get_env_fsm::Fsm as env_fsm;
use log::{trace, warn};
use openssh::{KnownHosts, Session};
use std::sync::Arc;
use telegram_bot::TelegramBot;
use thinkpad_dock_control_fsm::Fsm as thinkpad_ctrl_fsm;
use tokio::{
    signal,
    sync::{broadcast, Mutex},
};

#[derive(Clone, Debug)]
pub struct GlobalState {
    am_i_home: bool,
}

#[derive(Clone, Debug)]
pub struct Context {
    env_sensor_address_port: String,
    systems: Vec<System>,
}

#[derive(Clone, Debug)]
struct System {
    user: String,
    ip: String,
    mac: Option<String>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct OfficeEnv {
    brightness: u32,
    temperature: u32,
    humidity: u32,
}

fn load_env_vars() -> Context {
    dotenv().ok();

    std::env::var("TELOXIDE_TOKEN").unwrap_or_else(|_| panic!("TELOXIDE_TOKEN must be set."));

    // Check for the expected env vars
    let env_var_map = vec![
        "ESP8266_ADDRESS_PORT",
        "SYRINX_VARS",
        "SNOWDOG_VARS",
        "THINKPADX1_VARS",
    ];

    let mut systems = vec![];
    let mut esp8266_addr_port = String::new();

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
                esp8266_addr_port = val;
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
                systems.push(System {
                    user,
                    ip,
                    mac: Some(mac),
                });
            }
            "SYRINX_VARS" | "THINKPADX1_VARS" => {
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
                systems.push(System {
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

    Context {
        env_sensor_address_port: esp8266_addr_port,
        systems,
    }
}

async fn check_pcs_access(systems: &[System]) -> Result<(), String> {
    // this can be written better
    let mut return_str: String = "Failed to ssh connect to systems:\n".to_string();
    let mut error: bool = false;
    for (i, sys) in systems.iter().enumerate() {
        let session_access: &str = &(sys.user.clone() + "@" + &sys.ip);
        if Session::connect(session_access, KnownHosts::Strict)
            .await
            .is_ok()
        {
        } else {
            return_str += &("\tsystem".to_owned() + &i.to_string() + &format!(": {sys:?}\n"));
            error = true;
        };
    }

    if error {
        Err(return_str)
    } else {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let global_state: Arc<Mutex<GlobalState>> =
        Arc::new(Mutex::new(GlobalState { am_i_home: false }));
    let ctx = load_env_vars();
    pretty_env_logger::init();

    if let Err(e) = check_pcs_access(&ctx.systems).await {
        warn!("Failed to check PC access. Error: {e}");
    }

    // set some default data on office_env
    let office_env = OfficeEnv {
        brightness: 0,
        temperature: 0,
        humidity: 0,
    };

    let office_env = Arc::new(Mutex::new(office_env));

    let mut env_fsm = env_fsm::new(ctx.clone(), office_env.clone());
    let mut thinkpad_ctrl_fsm = thinkpad_ctrl_fsm::new(ctx.systems[1].clone(), office_env.clone());
    let telegram_bot = TelegramBot::new(ctx.clone(), global_state.clone());

    // Create a broadcast channel for shutdown signal
    let (shutdown_tx, _) = broadcast::channel(1);

    // Task 1: Office environment FSM
    let mut shutdown_rx1 = shutdown_tx.subscribe(); // Subscribe to the shutdown signal
    let handle1 = tokio::spawn(async move {
        tokio::select! {
            () = env_fsm.run() => {},
            _ = shutdown_rx1.recv() => {
                trace!("env_fsm received shutdown signal");
            }
        }
    });

    // Task 2: Desktop control FSM
    let mut shutdown_rx2 = shutdown_tx.subscribe(); // Subscribe to the shutdown signal
    let handle2 = tokio::spawn(async move {
        tokio::select! {
            () = thinkpad_ctrl_fsm.run() => {},
            _ = shutdown_rx2.recv() => {
                trace!("desktop_ctrl_fsm received shutdown signal");
            }
        }
    });

    // Task 3: Telegram bot
    let mut shutdown_rx3 = shutdown_tx.subscribe(); // Subscribe to the shutdown signal
    let handle3 = tokio::spawn(async move {
        tokio::select! {
            () = telegram_bot.run() => {},
            _ = shutdown_rx3.recv() => {
                trace!("telegram_bot received shutdown signal");
            }
        }
    });

    // Task 4: Am I at home?
    let mut shutdown_rx4 = shutdown_tx.subscribe(); // Subscribe to the shutdown signal
    let handle4 = tokio::spawn(async move {
        tokio::select! {
            () = commands::get_am_i_home(global_state.clone()) => {},
            _ = shutdown_rx4.recv() => {
                trace!("telegram_bot received shutdown signal");
            }
        }
    });

    // Task 5: Listen for Ctrl-C and broadcast the shutdown signal
    let shutdown_listener = tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl-C");
        trace!("Ctrl-C received, sending shutdown signal...");
        let _ = shutdown_tx.send(()); // Broadcast the shutdown signal
    });

    // Await all tasks to complete
    let _ = tokio::try_join!(handle1, handle2, handle3, handle4, shutdown_listener);
}
