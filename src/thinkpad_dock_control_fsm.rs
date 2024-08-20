use crate::commands::send_command;
use crate::OfficeEnv;
use crate::System;
use log::error;
use log::trace;
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
    Disconnected,
}

pub struct Fsm {
    state: State,
    system: System,
    env_data: Arc<Mutex<OfficeEnv>>,
    session: Option<Session>,
}

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
fn get_thinkpad_x1_mon_brightness(env_brightness: u32) -> u32 {
    let env_brightness = f64::from(env_brightness);
    let max_mon_brightness: f64 = 19393.0;
    let coef = 0.142_857_15;

    if env_brightness <= 50.0 {
        return 1000;
    }

    (((env_brightness * coef * max_mon_brightness) as u32) / 100)
        .clamp(1000, max_mon_brightness as u32)
}

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
fn get_main_mon_brightness(env_brightness: u32) -> u32 {
    let env_brightness = f64::from(env_brightness);
    let coef = 0.142_857_15;

    if env_brightness <= 50.0 {
        return 0;
    }

    ((env_brightness * coef) as u32).clamp(0, 100)
}

/// Returns `Err()` if the calcuated brightness is the same as the previous one
fn get_brightness_cmds(env_brightness: u32) -> Result<String, ()> {
    static MAIN_MON_BRIGHTNESS: AtomicU32 = AtomicU32::new(0);
    trace!("Environment brightness: {:?}", env_brightness);

    // only send command if calculated brightness is different than the previously sent one
    let main_mon_brightness = get_main_mon_brightness(env_brightness);
    if MAIN_MON_BRIGHTNESS.load(Ordering::Relaxed) == main_mon_brightness {
        trace!("Same brightness calculated. Static brightness: {MAIN_MON_BRIGHTNESS:?}, brightness: {main_mon_brightness}");
        return Err(());
    }

    MAIN_MON_BRIGHTNESS.store(main_mon_brightness, Ordering::SeqCst);

    let laptop_mon_brightness = get_thinkpad_x1_mon_brightness(env_brightness);

    let set_kbd_brightness = match env_brightness {
        250..=u32::MAX => 0,
        100..=249 => 2,
        0..=99 => 1,
    };

    let command: String = String::from("echo ")
        + &laptop_mon_brightness.to_string()
        + " > /sys/class/backlight/intel_backlight/brightness & ddcutil --bus 14 setvcp 10 "
        + &main_mon_brightness.to_string()
        + " & echo "
        + &set_kbd_brightness.to_string()
        + " > /sys/class/leds/tpacpi::kbd_backlight/brightness";

    Ok(command)
}

impl Fsm {
    async fn connecting(&mut self) {
        trace!("Connecting");
        let session_access: &str = &(self.system.user.clone() + "@" + &self.system.ip);
        trace!("Attempting ssh connect to {}", session_access);

        match Session::connect(session_access, KnownHosts::Strict).await {
            Ok(sesh) => {
                self.session = Some(sesh);
                self.state = State::Connected;
            }
            Err(e) => {
                trace!("Failed ssh connection to {0}. Error: {e}", self.system.ip);
                sleep(Duration::from_secs(2)).await;
            }
        }
    }

    async fn connected(&mut self) {
        let Some(session) = &self.session else {
            trace!("Not connected");
            self.state = State::Disconnected;
            return;
        };

        let env_brightness = self.env_data.lock().await.brightness;

        let command = if let Ok(cmd) = get_brightness_cmds(env_brightness) {
            trace!("Sending command: {cmd}");
            cmd
        } else {
            sleep(Duration::from_millis(500)).await;
            return;
        };

        match send_command(&command, Some(session)).await {
            Ok(out) => {
                trace!("{out}");
            }
            Err(e) => {
                error!(
                    "Failed sending brightness command:\n\t{0}\n\tError: {e}",
                    &command
                );
            }
        }
    }

    fn disconnected(&mut self) {
        trace!("Disconnected");
        self.state = State::Connecting;
        self.session = None;
    }

    pub async fn run(&mut self) {
        loop {
            match self.state {
                State::Connecting => self.connecting().await,
                State::Connected => self.connected().await,
                State::Disconnected => self.disconnected(),
            }
        }
    }

    pub fn new(sys: System, env_data: Arc<Mutex<OfficeEnv>>) -> Self {
        Self {
            state: State::Disconnected,
            system: sys,
            env_data,
            session: None,
        }
    }
}
