mod commands;
mod office_env_fsm;
mod telegram_bot;
use std::collections::HashMap;

use dotenv::dotenv;
use office_env_fsm::Fsm;
use telegram_bot::TelegramBot;

#[derive(Clone)]
struct System {
    ip: String,
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
        ("SYSTEM0", String::new()),
        ("SYSTEM1", String::new()),
        ("SYSTEM2", String::new()),
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
        systems: vec![System {
            ip: env_var_map
                .get("SYSTEM0")
                .expect("Why is this empty?")
                .to_string(),
        }],
    }

    //println!("Production Url: {}", &esp8266_address);
}

#[tokio::main]
async fn main() {
    let ctx = load_env_vars();

    let mut office_env_fsm = Fsm::new(ctx.clone());
    let telegram_bot = TelegramBot::new(ctx.clone());

    let handle1 = tokio::spawn(async move {
        office_env_fsm.run().await;
    });

    let handle2 = tokio::spawn(async move {
        telegram_bot.run().await;
    });

    // Await both tasks to complete
    let _ = handle1.await;
    let _ = handle2.await;
}
