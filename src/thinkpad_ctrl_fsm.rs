use crate::commands;
use crate::commands::send_command;
use crate::GlobalState;
use crate::Pc;
use log::debug;
use log::error;
use log::warn;
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

/// Returns `Err()` if the calcuated brightness is the same as the previous one
fn get_brightness_cmds(env_brightness: u32) -> Result<String, ()> {
    static MAIN_MON_BRIGHTNESS: AtomicU32 = AtomicU32::new(0);
    debug!("Environment brightness: {:?}", env_brightness);

    // only send command if calculated brightness is different than the previously sent one
    let main_mon_brightness = commands::calculate_ddc_mon_brightness(env_brightness);
    if MAIN_MON_BRIGHTNESS.load(Ordering::Relaxed) == main_mon_brightness {
        debug!("Same brightness calculated. Static brightness: {MAIN_MON_BRIGHTNESS:?}, brightness: {main_mon_brightness}");
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
        debug!("Connecting");
        let session_access: &str = &(self.pc.user.clone() + "@" + &self.pc.ip);
        debug!("Attempting ssh connect to {}", session_access);

        match Session::connect(session_access, KnownHosts::Strict).await {
            Ok(sesh) => {
                self.session = Some(sesh);
                self.state = State::Connected;
            }
            Err(e) => {
                warn!("Failed ssh connection to {0}. Error: {e}", self.pc.ip);
                sleep(Duration::from_secs(2)).await;
            }
        }
    }

    async fn connected(&mut self) {
        let Some(session) = &self.session else {
            debug!("Not connected");
            self.state = State::Connecting;
            return;
        };

        let env_brightness = self.global_state.lock().await.office_env.brightness;

        let command = if let Ok(cmd) = get_brightness_cmds(env_brightness) {
            debug!("Sending command: {cmd}");
            cmd
        } else {
            sleep(Duration::from_millis(500)).await;
            return;
        };

        match send_command(&command, Some(session)).await {
            Ok(out) => {
                debug!("{out}");
            }
            Err(e) => {
                error!(
                    "Failed sending brightness command:\n\t{0}\n\tError: {e}",
                    &command
                );
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
