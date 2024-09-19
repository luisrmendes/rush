use crate::commands::calculate_ddc_mon_brightness;
use crate::commands::send_command;
use crate::GlobalState;
use crate::Pc;
use log::debug;
use log::error;
use openssh::KnownHosts;
use openssh::Session;
use ping_rs::send_ping_async;
use regex::Regex;
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
    async fn get_i2c_monitor_numbers(session: &Session) -> Result<Vec<String>, String> {
        // get available monitors
        let monitor_list_out = send_command("ddcutil detect", Some(session)).await?;

        // parse monitor bus'
        let re = Regex::new(r"/dev/i2c-(\d+)").unwrap();

        // Find all matches and capture the last digit part
        let matches: Vec<String> = re
            .captures_iter(&monitor_list_out) // Iterate over all captures
            .map(|cap| cap[1].to_string()) // Capture group 1 contains the digit after `/dev/i2c-`
            .collect();

        Ok(matches)
    }

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

        let monitor_i2c_numbers = match Self::get_i2c_monitor_numbers(session).await {
            Ok(out) => out,
            Err(e) => {
                error!("{e}");
                sleep(Duration::from_millis(500)).await;
                return;
            }
        };

        let env_brightness = self.global_state.lock().await.office_env.brightness;
        let mon_brightness = calculate_ddc_mon_brightness(env_brightness);

        // only send command if calculated brightness is different than the previously sent one
        if MON_BRIGHTNESS.load(Ordering::Relaxed) == mon_brightness {
            sleep(Duration::from_millis(500)).await;
            return;
        }

        MON_BRIGHTNESS.store(mon_brightness, Ordering::SeqCst);

        // build command string
        let mut command_builder = String::new();
        for i2c_num in monitor_i2c_numbers {
            command_builder += &format!("ddcutil --bus {i2c_num} setvcp 10 {mon_brightness} & ");
        }

        match send_command(&command_builder, Some(session)).await {
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
