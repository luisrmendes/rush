use crate::OfficeEnv;
use crate::System;
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

fn get_thinkpad_x1_mon_brightness(env_brightness: u32) -> u32 {
    let env_brightness = env_brightness as f32;
    let max_mon_brightness: f32 = 19393.0;
    let coef = 0.14285714285714285;

    if env_brightness <= 50.0 {
        return 1000;
    }

    return ((env_brightness * coef * max_mon_brightness) as u32) / 100;
}

fn get_main_mon_brightness(env_brightness: u32) -> u32 {
    let env_brightness = env_brightness as f32;
    let coef = 0.14285714285714285;

    if env_brightness <= 50.0 {
        return 0;
    }

    return (env_brightness * coef) as u32;
}

async fn send_brightness_cmds(env_brightness: u32, session: &Session) -> Result<(), ()> {
    // only send command if calculated brightness is different than the previously sent one
    static MAIN_MON_BRIGHTNESS: AtomicU32 = AtomicU32::new(0);
    let main_mon_brightness = get_main_mon_brightness(env_brightness);
    if MAIN_MON_BRIGHTNESS.load(Ordering::SeqCst) != main_mon_brightness {
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
        + " > /sys/class/backlight/intel_backlight/brightness & ddcutil --bus 14 setvcp 10 & "
        + &main_mon_brightness.to_string()
        + "echo "
        + &set_kbd_brightness.to_string()
        + " > /sys/class/leds/tpacpi::kbd_backlight/brightness";

    match session.shell(command).output().await {
        Ok(output) => output,
        Err(e) => {
            println!("Error: {e:?}");
            return Err(());
        }
    };

    Ok(())
}

impl Fsm {
    async fn connecting(&mut self) {
        println!("Attempting ssh connect to {}", self.system.ip);

        match Session::connect(self.system.ip.clone(), KnownHosts::Strict).await {
            Ok(sesh) => {
                self.session = Some(sesh);
                self.state = State::Connected;
            }
            Err(e) => {
                println!("Failed ssh connection to {0}. Error: {e}", self.system.ip);
                sleep(Duration::from_secs(2)).await;
                return;
            }
        };
    }

    async fn connected(&mut self) {
        let Some(session) = &mut self.session else {
            println!("Not connected");
            self.state = State::Disconnected;
            return;
        };

        let env_brightness = self.env_data.lock().await.brightness;
        println!("{:?}", env_brightness);

        if send_brightness_cmds(env_brightness, &session)
            .await
            .is_err()
        {
            sleep(Duration::from_secs(2)).await;
            return;
        }
    }

    fn disconnected(&mut self) {
        println!("Disconnected");
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
            env_data: env_data,
            session: None,
        }
    }
}
