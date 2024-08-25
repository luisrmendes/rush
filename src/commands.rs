use crate::{GlobalState, System};
use log::{trace, warn};
use openssh::{KnownHosts, Session};
use std::{
    sync::{
        atomic::{AtomicI16, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{process::Command, sync::Mutex, time::sleep};

/// Polling function that updates global state if i am at home or nor
/// I am at home if my phone is connected to the local network
/// Assumes i am not at home if my phone is not in the network in three consecutive searches
pub async fn get_am_i_home(global_state: Arc<Mutex<GlobalState>>) {
    loop {
        static AM_I_NOT_AT_HOME_COUNTER: AtomicI16 = AtomicI16::new(0);
        let am_i_at_home = global_state.lock().await.am_i_home;

        match send_command("nmap -T5 -sn 192.168.1.0/24", None).await {
            Ok(out) => {
                if out.contains("unknownbc7fa4343f2d") {
                    global_state.lock().await.am_i_home = true;
                } else if am_i_at_home {
                    // increments the counter if does not find my phone
                    AM_I_NOT_AT_HOME_COUNTER.fetch_add(1, Ordering::Relaxed);
                    trace!(
                        "adding 1 to not at home counter. Counter: {}",
                        AM_I_NOT_AT_HOME_COUNTER.load(Ordering::Relaxed)
                    );
                }
            }
            Err(e) => {
                println!("Send command error: {e}");
            }
        };

        if global_state.lock().await.am_i_home
            && AM_I_NOT_AT_HOME_COUNTER.load(Ordering::Relaxed) >= 3
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
    trace!("Sending command: {}", command);
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
            Err(e) => Err(format!("Failed to execute command. Error: {e}")),
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

                trace!("stdout: \n{stdout}");
                trace!("stderr: \n{stderr}");

                if !stderr.is_empty() {
                    return Err(stderr);
                }

                Ok(stdout)
            }
            Err(e) => Err(format!("Failed to execute command. Error: {e}")),
        },
    }
}

pub async fn suspend(sys: System) -> Result<String, String> {
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

pub fn is_online(target_sys: &System) -> bool {
    ping_rs::send_ping(
        &target_sys.ip.parse().unwrap(),
        Duration::from_millis(100),
        &[1, 2, 3, 4],
        None,
    )
    .is_ok()
}

pub async fn wakeup(target_sys: System) -> Result<String, String> {
    let Some(mac) = target_sys.mac else {
        return Err("Trying to wakup a system without a associated MAC address".to_string());
    };

    send_command(&format!("wol {}", &mac), None).await
}

pub async fn get_ipv4() -> Result<String, String> {
    send_command(
        "dig @resolver1.opendns.com A myip.opendns.com +short -4",
        None,
    )
    .await
}
