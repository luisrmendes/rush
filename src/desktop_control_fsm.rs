use std::sync::Arc;

use openssh::KnownHosts;
use openssh::Session;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio::time::timeout;
use tokio::time::Duration;

use crate::Context;
use crate::OfficeEnv;
use crate::System;
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

fn get_laptop_mon_brightness(env_brightness: u32) -> u32 {
    return 0;
}

fn get_main_mon_brightness(env_brightness: u32) -> u32 {
    return 0;
}

impl Fsm {
    pub fn new(sys: System, env_data: Arc<Mutex<OfficeEnv>>) -> Self {
        Self {
            state: State::Disconnected,
            system: sys,
            env_data: env_data,
            session: None,
        }
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

    async fn connecting(&mut self) {
        println!("Attempting to connect to {}", self.system.ip);

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
            println!("Failed to connect");
            self.state = State::Disconnected;
            return;
        };

        println!("{:?}", self.env_data.lock().await);

        let _laptop_mon_brightness =
            get_laptop_mon_brightness(self.env_data.lock().await.brightness);
        let _main_mon_brightness =
            get_main_mon_brightness(self.env_data.lock().await.brightness);

        // let command = "echo 1000 > /sys/class/backlight/intel_backlight/brightness & ddcutil --bus 14 setvcp 10 30";
        let command = "ls && ls";

        let output = match session.command("ls").output().await {
        // let output = match session.command("echo").arg("1000").arg(">").arg("/sys/class/backlight/intel_backlight/brightness").output().await {
            Ok(output) => output,
            Err(e) => {
                println!("Error: {e:?}");
                sleep(Duration::from_secs(2)).await;
                return;
            }
        };

        let output = match String::from_utf8(output.stdout) {
            Ok(output) => output,
            Err(e) => {
                println!("Error: {e}");
                sleep(Duration::from_secs(2)).await;
                return;
            }
        };

        println!("{output}");

        sleep(Duration::from_secs(2)).await;
    }

    fn disconnected(&mut self) {
        println!("Disconnected");
        self.state = State::Connecting;
        self.session = None;
    }
}
