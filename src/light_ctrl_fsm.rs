use chrono::Local;
use chrono::Timelike;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::sleep;

use tokio::time::Duration;

use crate::commands;
use crate::GlobalState;

enum State {
    Connecting,
    Connected,
}

#[allow(dead_code)]
pub struct Fsm {
    state: State,
    global_state: Arc<Mutex<GlobalState>>,
}

static FSM_REST: Duration = Duration::new(5, 0); // rest time between states

impl Fsm {
    pub fn new(global_state: Arc<Mutex<GlobalState>>) -> Self {
        Self {
            state: State::Connecting,
            global_state,
        }
    }

    pub async fn run(&mut self) {
        loop {
            match self.state {
                State::Connecting => self.connecting(),
                State::Connected => self.connected().await,
            }
        }
    }

    fn connecting(&mut self) {
        self.state = State::Connected;
    }

    async fn connected(&mut self) {
        let now = Local::now();
        if now.hour() == 18 && now.minute() == 0 {
            let client = Client::new();

            let _response = client
                .get("http://".to_owned() + commands::SHELLY_PLUG4_HOSTNAME + "/relay/0?turn=on")
                .send()
                .await;
            sleep(Duration::new(1, 0)).await;
            let _response = client
                .get("http://".to_owned() + commands::SHELLY_PLUG5_HOSTNAME + "/relay/0?turn=on")
                .send()
                .await;
            sleep(Duration::new(1, 0)).await;
            let _response = client
                .get("http://".to_owned() + commands::SHELLY_PLUG6_HOSTNAME + "/relay/0?turn=ofonf")
                .send()
                .await;
        } else {
            sleep(FSM_REST).await;
        }

        if now.hour() == 23 && now.minute() == 0 {
            let _ = commands::lights_off_living_room().await;
        } else {
            sleep(FSM_REST).await;
        }
    }
}
