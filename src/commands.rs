use crate::{GlobalState, Pc};
use log::{debug, error, warn};
use openssh::{KnownHosts, Session, SessionBuilder};
use reqwest::Client;
use std::error::Error;
use std::net::ToSocketAddrs;
use std::{
    sync::{
        atomic::{AtomicI16, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{process::Command, sync::Mutex, time::sleep};

pub(crate) static SHELLY_PLUG1_HOSTNAME: &str = "shellyplusplugs-fcb4670e1130";
pub(crate) static SHELLY_PLUG2_HOSTNAME: &str = "shellyplusplugs-e465b8b362f8";
pub(crate) static SHELLY_PLUG3_HOSTNAME: &str = "shellyplusplugs-c82e180a8dd8";
pub(crate) static SHELLY_PLUG4_HOSTNAME: &str = "shellyplusplugs-c82e180b59c4";
pub(crate) static SHELLY_PLUG5_HOSTNAME: &str = "shellyplusplugs-c82e18083148";
pub(crate) static SHELLY_PLUG6_HOSTNAME: &str = "shellyplusplugs-fcb4670d686c";

pub async fn lights_off_living_room() -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    let response = client
        .get("http://".to_owned() + SHELLY_PLUG4_HOSTNAME + "/relay/0?turn=off")
        .send()
        .await?;

    let _response = client
        .get("http://".to_owned() + SHELLY_PLUG5_HOSTNAME + "/relay/0?turn=off")
        .send()
        .await?;

    let _response = client
        .get("http://".to_owned() + SHELLY_PLUG6_HOSTNAME + "/relay/0?turn=off")
        .send()
        .await?;

    Ok(response.text().await?)
}

pub async fn lights_on_living_room() -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    // let response = client
    //     .get("http://".to_owned() + SHELLY_PLUG1_HOSTNAME + "/relay/0?turn=on")
    //     .send()
    //     .await?;

    // let _response = client
    //     .get("http://".to_owned() + SHELLY_PLUG2_HOSTNAME + "/relay/0?turn=on")
    //     .send()
    //     .await?;

    // let _response = client
    //     .get("http://".to_owned() + SHELLY_PLUG3_HOSTNAME + "/relay/0?turn=on")
    //     .send()
    //     .await?;

    let response = client
        .get("http://".to_owned() + SHELLY_PLUG4_HOSTNAME + "/relay/0?turn=on")
        .send()
        .await?;

    let _response = client
        .get("http://".to_owned() + SHELLY_PLUG5_HOSTNAME + "/relay/0?turn=on")
        .send()
        .await?;

    let _response = client
        .get("http://".to_owned() + SHELLY_PLUG6_HOSTNAME + "/relay/0?turn=on")
        .send()
        .await?;

    Ok(response.text().await?)
}

pub async fn lights_off() -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get("http://".to_owned() + SHELLY_PLUG1_HOSTNAME + "/relay/0?turn=off")
        .send()
        .await?;

    let _response = client
        .get("http://".to_owned() + SHELLY_PLUG2_HOSTNAME + "/relay/0?turn=off")
        .send()
        .await?;

    let _response = client
        .get("http://".to_owned() + SHELLY_PLUG3_HOSTNAME + "/relay/0?turn=off")
        .send()
        .await?;

    let _response = client
        .get("http://".to_owned() + SHELLY_PLUG4_HOSTNAME + "/relay/0?turn=off")
        .send()
        .await?;

    let _response = client
        .get("http://".to_owned() + SHELLY_PLUG5_HOSTNAME + "/relay/0?turn=off")
        .send()
        .await?;

    let _response = client
        .get("http://".to_owned() + SHELLY_PLUG6_HOSTNAME + "/relay/0?turn=off")
        .send()
        .await?;

    Ok(response.text().await?)
}

pub async fn lights_on() -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get("http://".to_owned() + SHELLY_PLUG1_HOSTNAME + "/relay/0?turn=on")
        .send()
        .await?;

    let _response = client
        .get("http://".to_owned() + SHELLY_PLUG2_HOSTNAME + "/relay/0?turn=on")
        .send()
        .await?;

    let _response = client
        .get("http://".to_owned() + SHELLY_PLUG3_HOSTNAME + "/relay/0?turn=on")
        .send()
        .await?;

    let _response = client
        .get("http://".to_owned() + SHELLY_PLUG4_HOSTNAME + "/relay/0?turn=on")
        .send()
        .await?;

    let _response = client
        .get("http://".to_owned() + SHELLY_PLUG5_HOSTNAME + "/relay/0?turn=on")
        .send()
        .await?;

    let _response = client
        .get("http://".to_owned() + SHELLY_PLUG6_HOSTNAME + "/relay/0?turn=on")
        .send()
        .await?;

    Ok(response.text().await?)
}

pub async fn get_ssh_status(target_pc: &Pc) -> bool {
    let session_access: String = target_pc.user.clone() + "@" + &target_pc.ip;
    let mut sesh_builder = SessionBuilder::default();
    sesh_builder.user(target_pc.user.clone());
    sesh_builder.connect_timeout(Duration::from_secs(1));

    sesh_builder.connect(session_access).await.is_ok()
}

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
pub fn calculate_ddc_mon_brightness(env_brightness: u32) -> u32 {
    let env_brightness = f64::from(env_brightness);
    let coef = 0.13;

    if env_brightness <= 50.0 {
        return 0;
    }

    ((env_brightness * coef) as u32).clamp(0, 100)
}

pub async fn check_external_system_connection(pcs: &[Pc]) -> Result<String, String> {
    debug!("checking for PC accesses");
    let mut return_str: String = String::new();
    let mut is_there_some_error: bool = false; // used to return Result Err

    // check if systems are online
    for pc in pcs {
        let online_status: &str = match is_online(pc) {
            Ok(true) => "online",
            Ok(false) => {
                is_there_some_error = true;
                "offline"
            }
            Err(e) => {
                is_there_some_error = true;
                &format!("Failed to lookup PC {0}. Error: {e}\n", pc.ip)
            }
        };

        let ssh_status: String = if get_ssh_status(pc).await {
            "Ok".to_owned()
        } else {
            is_there_some_error = true;
            format!("Failed ssh access. \n\tSystem: {pc:?}")
        };

        return_str += &format!(
            "\nSystem \'{}\' is {online_status}, ssh status: {ssh_status}",
            &pc.ip
        );
    }

    if is_there_some_error {
        Err(return_str)
    } else {
        Ok(return_str)
    }
}

/// Polling function that updates global state if i am at home or nor
/// I am at home if my phone is connected to the local network
/// Assumes i am not at home if my phone is not in the network in three consecutive searches
/// TODO: Check if dependencies exist. If not, install (nmap)
pub async fn get_am_i_home(global_state: Arc<Mutex<GlobalState>>) {
    loop {
        static AM_I_NOT_AT_HOME_COUNTER: AtomicI16 = AtomicI16::new(0);
        let am_i_at_home = global_state.lock().await.am_i_home;

        match send_command("nmap -T5 -sn 192.168.1.0/24", None).await {
            Ok(out) => {
                if out.contains("Galaxy-S22") {
                    global_state.lock().await.am_i_home = true;
                    AM_I_NOT_AT_HOME_COUNTER.store(0, Ordering::SeqCst);
                } else if am_i_at_home {
                    // increments the counter if does not find my phone
                    AM_I_NOT_AT_HOME_COUNTER.fetch_add(1, Ordering::Relaxed);
                    debug!(
                        "adding 1 to not at home counter. Counter: {}",
                        AM_I_NOT_AT_HOME_COUNTER.load(Ordering::Relaxed)
                    );
                }
            }
            Err(e) => {
                error!("Send command error: {e}");
            }
        };

        if global_state.lock().await.am_i_home
            && AM_I_NOT_AT_HOME_COUNTER.load(Ordering::Relaxed) > 30
        {
            global_state.lock().await.am_i_home = false;
            AM_I_NOT_AT_HOME_COUNTER.store(0, Ordering::SeqCst);
        }
        sleep(Duration::from_secs(1)).await;
    }
}

/// Simple wrapper with the trace logs
/// Runs in ssh if receives an ssh session, locally if not
pub async fn send_command(command: &str, ssh_session: Option<&Session>) -> Result<String, String> {
    debug!("Sending command: {}", command);
    match ssh_session {
        Some(sesh) => match sesh.shell(command).output().await {
            Ok(out) => {
                let stdout = if let Ok(out) = String::from_utf8(out.stdout.clone()) {
                    out
                } else {
                    warn!("Falied to parse stdout into UTF-8!");
                    String::new()
                };

                let stderr = if let Ok(err) = String::from_utf8(out.stderr.clone()) {
                    err
                } else {
                    warn!("Falied to parse stderr into UTF-8!");
                    String::new()
                };

                if !stderr.is_empty() {
                    return Err(stderr);
                }

                Ok(stdout)
            }
            Err(e) => Err(format!(
                "Failed to execute command \'{command}\'. Error: {e}"
            )),
        },
        None => match Command::new("sh").arg("-c").arg(command).output().await {
            Ok(out) => {
                let stdout = if let Ok(out) = String::from_utf8(out.stdout.clone()) {
                    out
                } else {
                    warn!("Falied to parse stdout into UTF-8!");
                    String::new()
                };

                let stderr = if let Ok(err) = String::from_utf8(out.stderr.clone()) {
                    err
                } else {
                    warn!("Falied to parse stderr into UTF-8!");
                    String::new()
                };

                debug!("stdout: \n{stdout}");
                debug!("stderr: \n{stderr}");

                if !stderr.is_empty() {
                    return Err(stderr);
                }

                Ok(stdout)
            }
            Err(e) => Err(format!("Failed to execute command. Error: {e}")),
        },
    }
}

#[allow(dead_code)]
pub async fn suspend(sys: Pc) -> Result<String, String> {
    let session_access: &str = &(sys.user.clone() + "@" + &sys.ip);
    let session = match Session::connect(session_access, KnownHosts::Strict).await {
        Ok(session) => session,
        Err(e) => {
            return Err(format!(
                "Failed ssh connection to {session_access}. Error: {e}"
            ));
        }
    };

    send_command("sudo systemctl suspend", Some(&session)).await
}

pub fn is_online(target_sys: &Pc) -> Result<bool, String> {
    // Resolve the hostname to an IP address
    let mut addr = match (target_sys.ip.clone(), 0).to_socket_addrs() {
        Ok(addr) => addr,
        Err(e) => return Err(format!("Error resolving hostname: {e}")),
    };

    let addr = match addr.next().ok_or("Failed to resolve hostname") {
        Ok(addr) => addr,
        Err(e) => return Err(e.to_string()),
    };

    Ok(ping_rs::send_ping(&addr.ip(), Duration::from_millis(100), &[1, 2, 3, 4], None).is_ok())
}

#[allow(dead_code)]
pub async fn wakeup(target_sys: Pc) -> Result<String, String> {
    let Some(mac) = target_sys.mac else {
        return Err("Trying to wakup a system without a associated MAC address".to_string());
    };

    send_command(&format!("wakeonlan {}", &mac), None).await
}

pub async fn get_ipv4() -> Result<String, String> {
    send_command(
        "dig @resolver1.opendns.com A myip.opendns.com +short -4",
        None,
    )
    .await
}
