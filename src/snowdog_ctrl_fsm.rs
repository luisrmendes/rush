use crate::commands::calculate_ddc_mon_brightness;
use crate::commands::send_command;
use crate::GlobalState;
use crate::Pc;
use log::debug;
use log::error;
use openssh::KnownHosts;
use openssh::Session;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio::time::Duration;

enum State {
    Connecting,
    Connected,
}

pub struct Fsm {
    state: State,
    pc: Pc,
    global_state: Arc<Mutex<GlobalState>>,
    session: Option<Session>,
}

impl Fsm {
    async fn connecting(&mut self) {
        debug!("Connecting");
        let session_access: &str = &(self.pc.user.clone() + "@" + &self.pc.ip);
        debug!("Attempting ssh connect to {}", session_access);

        match Session::connect(session_access, KnownHosts::Strict).await {
            Ok(sesh) => {
                self.session = Some(sesh);
                self.state = State::Connected;
            }
            Err(e) => {
                debug!("Failed ssh connection to {0}. Error: {e}", self.pc.ip);
                sleep(Duration::from_secs(2)).await;
            }
        }
    }

    async fn connected(&mut self) {
        static MON_BRIGHTNESS: AtomicU32 = AtomicU32::new(0);
        let Some(session) = &self.session else {
            debug!("Not connected");
            self.state = State::Connecting;
            return;
        };

        let env_brightness = self.global_state.lock().await.office_env.brightness;
        let mon_brightness = calculate_ddc_mon_brightness(env_brightness);

        // only send command if calculated brightness is different than the previously sent one
        if MON_BRIGHTNESS.load(Ordering::Relaxed) == mon_brightness {
            debug!("Same brightness calculated. Static brightness: {MON_BRIGHTNESS:?}, brightness: {mon_brightness}");
            sleep(Duration::from_millis(500)).await;
            return;
        }

        MON_BRIGHTNESS.store(mon_brightness, Ordering::SeqCst);

        let command: String = format!("ddcutil --bus 5 setvcp 10 {mon_brightness}");

        match send_command(&command, Some(session)).await {
            Ok(out) => {
                debug!("{out}");
            }
            Err(e) => {
                error!("{e}");
            }
        }
    }

    pub async fn run(&mut self) {
        loop {
            match self.state {
                State::Connecting => self.connecting().await,
                State::Connected => self.connected().await,
            }
        }
    }

    pub fn new(pc: Pc, global_state: Arc<Mutex<GlobalState>>) -> Self {
        Self {
            state: State::Connecting,
            pc,
            global_state,
            session: None,
        }
    }
}
