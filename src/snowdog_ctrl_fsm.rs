use crate::commands::calculate_ddc_mon_brightness;
use crate::commands::send_command;
use crate::GlobalState;
use crate::Pc;
use log::debug;
use log::error;
use log::warn;
use openssh::KnownHosts;
use openssh::Session;
use regex::Regex;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
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
    monitor_i2c_ids: Arc<Mutex<Vec<String>>>,
}

impl Fsm {
    async fn get_i2c_monitor_numbers(session: &Session) -> Result<Vec<String>, String> {
        let monitor_list_out = match send_command("ddcutil detect", Some(session)).await {
            Ok(out) => out,
            Err(e) => {
                return Err(format!("{e}"));
            }
        };

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
                return;
            }
        }

        let Some(session) = &mut self.session else {
            sleep(Duration::from_secs(2)).await;
            return;
        };

        let monitor_i2c_ids = match Self::get_i2c_monitor_numbers(session).await {
            Ok(out) => out,
            Err(e) => {
                warn!("Failed to get monitor i2c ids. Error: {e}");
                return;
            }
        };

        *self.monitor_i2c_ids.lock().await = monitor_i2c_ids;
    }

    async fn connected(&mut self) {
        static MON_BRIGHTNESS: AtomicU32 = AtomicU32::new(0);
        static GET_I2C_COUNTER: AtomicU32 = AtomicU32::new(0);

        //let session_arc: Arc<Mutex<Option<Session>>> = Arc::new(Mutex::new(self.session));

        let env_brightness = self.global_state.lock().await.office_env.brightness;
        let mon_brightness = calculate_ddc_mon_brightness(env_brightness);

        // Only proceed if brightness has changed
        if MON_BRIGHTNESS.load(Ordering::Relaxed) == mon_brightness {
            debug!("Same brightness calculated. Static brightness: {MON_BRIGHTNESS:?}, brightness: {mon_brightness}");
            sleep(Duration::from_millis(500)).await;
            return;
        }

        MON_BRIGHTNESS.store(mon_brightness, Ordering::SeqCst);

        // Clone Arc references so they can be accessed inside the async task
        let monitor_i2c_ids = Arc::clone(&self.monitor_i2c_ids);

        // refresh i2c monitor numbers periodically in case i've switched a monitor off or on
        // GET_I2C_COUNTER.fetch_add(1, Ordering::SeqCst);
        // if GET_I2C_COUNTER.load(Ordering::SeqCst) >= 20 {
        //     GET_I2C_COUNTER.store(0, Ordering::SeqCst);

        //     tokio::spawn(async move {
        //         loop {
        //             let mut monitor_i2c_ids_lock = monitor_i2c_ids.lock().await;
        //             let session_lock = session_arc.lock().await;

        //             let Some(ref session) = *session_lock else {
        //                 debug!("Not connected");
        //                 return;
        //             };

        //             let i2c = match Self::get_i2c_monitor_numbers(&session).await {
        //                 Ok(out) => out,
        //                 Err(e) => {
        //                     warn!("Failed to get monitor I2C ids. Error: {e}");
        //                     sleep(Duration::from_secs(30)).await;
        //                     continue;
        //                 }
        //             };

        //             debug!("Refreshing I2C IDs: {i2c:?}");
        //             *monitor_i2c_ids_lock = i2c;
        //         }
        //     });
        // }

        // Use the original `session` here
        let mut command_builder = String::new();
        for i2c_num in &*self.monitor_i2c_ids.lock().await {
            command_builder += &format!("ddcutil --bus {i2c_num} setvcp 10 {mon_brightness} & ");
        }

        debug!("Sending command: {command_builder}");

        // Check for session, otherwise reconnect
        // let session_mutex2 = Arc::clone(&session_arc);

        let Some(session) = &self.session else {
            debug!("Not connected");
            self.state = State::Connecting;
            return;
        };

        // Use the original session to send the command
        match send_command(&command_builder, Some(session)).await {
            Ok(out) => {
                debug!("Command output: {out}");
            }
            Err(e) => {
                error!("Failed to send command: {e}");
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
            monitor_i2c_ids: Arc::new(Mutex::new(vec![])),
        }
    }
}
