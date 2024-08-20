use crate::System;
use log::trace;
use openssh::{KnownHosts, Session};
use std::time::Duration;
use tokio::process::Command;

#[derive(Debug)]
pub enum Operation {
    GetIpv4,
    WakeupDesktop,
    StatusDesktop,
    SuspendDesktop,
}

/// Simple wrapper with the trace logs
/// Runs in ssh if receives an ssh session, locally if not
pub async fn send_command(command: &str, ssh_session: Option<&Session>) -> Result<String, String> {
    trace!("Sending command: {}", command);
    match ssh_session {
        Some(sesh) => {
            match sesh.shell(command).output().await {
                Ok(out) => {
                    let stdout = String::from_utf8(out.stdout.clone()).expect("");
                    let stderr = String::from_utf8(out.stderr.clone()).expect("");

                    if !stderr.is_empty() {
                        return Err(stderr);
                    }

                    Ok(stdout)
                }
                Err(e) => {
                    Err(format!("Failed to execute command. Error: {e}"))
                }
            }
        }
        None => {
            match Command::new("sh").arg("-c").arg(command).output().await {
                Ok(out) => {
                    let stdout = String::from_utf8(out.stdout.clone()).expect("");
                    let stderr = String::from_utf8(out.stderr.clone()).expect("");
                    trace!("stdout: {stdout}");
                    trace!("stderr: {stderr}");

                    if !stderr.is_empty() {
                        return Err(stderr);
                    }

                    Ok(stdout)
                }
                Err(e) => {
                    Err(format!("Failed to execute command. Error: {e}"))
                }
            }
        }
    }
}

pub async fn suspend(sys: System) -> Result<String, String> {
    let session_access: &str = &(sys.user.to_owned() + "@" + &sys.ip);
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

pub async fn is_online(target_sys: System) -> bool {
    ping_rs::send_ping(
        &target_sys.ip.parse().unwrap(),
        Duration::from_millis(100),
        &[1, 2, 3, 4],
        None,
    )
    .is_ok()
}

pub async fn wakeup(target_sys: System) -> Result<String, String> {
    let mac = match target_sys.mac {
        Some(mac) => mac,
        None => {
            return Err("Trying to wakup a system without a associated MAC address".to_string());
        }
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
