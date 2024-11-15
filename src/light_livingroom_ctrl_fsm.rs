use chrono::Local;
use chrono::Timelike;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::sleep;

use tokio::time::Duration;

use crate::commands;
use crate::commands::LightCmd;
use crate::GlobalState;

enum State {
    Connecting,
    Connected,
}

pub struct Fsm {
    state: State,
    global_state: Arc<Mutex<GlobalState>>,
    are_living_room_lights_on: bool,
}

static FSM_REST: Duration = Duration::new(5, 0); // rest time between states

impl Fsm {
    pub fn new(global_state: Arc<Mutex<GlobalState>>) -> Self {
        Self {
            state: State::Connecting,
            global_state,
            are_living_room_lights_on: false,
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
        if !(self.global_state.lock().await.am_i_home || self.global_state.lock().await.is_she_home) {
            sleep(FSM_REST).await;
            return;
        }

        let now = Local::now();
        if now.hour() == 18 && now.minute() == 30 && !self.are_living_room_lights_on {
            let _ = commands::ctrl_bulb(&commands::VINTAGE_BULB, LightCmd::On).await;
            self.are_living_room_lights_on = true;
        } else {
            sleep(FSM_REST).await;
        }

        if now.hour() == 0 && now.minute() == 0 {
            let _ = commands::ctrl_hall_lights(LightCmd::Off).await;
            self.are_living_room_lights_on = false;
        } else {
            sleep(FSM_REST).await;
        }
    }
}
