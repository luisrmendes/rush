use std::{
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

use crate::System;
use openssh::{KnownHosts, Session};
use tokio::{net::TcpListener, process::Command};

#[derive(Debug)]
pub enum Operation {
    GetIpv4,
    WakeupDesktop,
    StatusDesktop,
    SuspendDesktop,
}

pub async fn suspend(sys: System) -> Result<String, String> {
    let session_access: &str = &(sys.user.to_owned() + "@" + &sys.ip);
    let session = match Session::connect(session_access, KnownHosts::Strict).await {
        Ok(session) => session,
        Err(e) => {
            return Err(format!(
                "Failed ssh connection to {0}. Error: {e}",
                session_access
            ));
        }
    };

    return match session.shell("sudo systemctl suspend").output().await {
        Ok(_) => Ok("Successful".to_string()),
        Err(e) => {
            return Err(format!("Failed to execute command. Error: {e}"));
        }
    };
}

pub async fn is_online(target_sys: System) -> bool {
    return ping_rs::send_ping(
        &target_sys.ip.parse().unwrap(),
        Duration::from_secs(1),
        &[1, 2, 3, 4],
        None,
    )
    .is_ok();
}

pub async fn wakeup(target_sys: System) -> Result<String, String> {
    let mac = match target_sys.mac {
        Some(mac) => mac,
        None => {
            return Err(format!(
                "Trying to wakup a system without a associated MAC address"
            ));
        }
    };

    match Command::new("sh")
        .arg("-c")
        .arg("wol ".to_string() + &mac)
        .output()
        .await
    {
        Ok(_) => return Ok("Success".to_string()),
        Err(e) => {
            return Err(format!("Failed to execute command. Error: {e}"));
        }
    };
}

pub async fn get_ipv4(sys: System) -> Result<String, String> {
    let session_access: &str = &(sys.user.to_owned() + "@" + &sys.ip);
    let session = match Session::connect(session_access, KnownHosts::Strict).await {
        Ok(session) => session,
        Err(e) => {
            return Err(format!(
                "Failed ssh connection to {0}. Error: {e}",
                session_access
            ));
        }
    };

    let output = match session
        .command("dig")
        .arg("@resolver1.opendns.com")
        .arg("A")
        .arg("myip.opendns.com")
        .arg("+short")
        .arg("-4")
        .output()
        .await
    {
        Ok(output) => output,
        Err(e) => {
            return Err(format!("Failed to execute command. Error: {e}"));
        }
    };

    let output = match String::from_utf8(output.stdout) {
        Ok(output) => output,
        Err(e) => {
            return Err(format!(
                "Failed to convert command output to UTF-8. Error: {e}"
            ));
        }
    };

    Ok(output)
}
