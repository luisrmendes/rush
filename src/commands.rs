use crate::System;
use openssh::{KnownHosts, Session};

#[derive(Debug)]
pub enum Operation {
    GetIpv4,
    WakeupDesktop,
}

pub async fn wakeup(executing_sys: System, target_sys: System) -> Result<String, String> {
    let session = match Session::connect(&executing_sys.ip, KnownHosts::Strict).await {
        Ok(session) => session,
        Err(e) => {
            return Err(format!("Failed ssh connection to {0}. Error: {e}", executing_sys.ip));
        }
    };

    let mac = match target_sys.mac {
        Some(mac) => mac,
        None => {
            return Err(format!(
                "Trying to wakup a system without a associated MAC address"
            ));
        }
    };

    let output = match session.command("wol").arg(mac).output().await {
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

pub async fn get_ipv4(sys: System) -> Result<String, String> {
    let session = match Session::connect(&sys.ip, KnownHosts::Strict).await {
        Ok(session) => session,
        Err(e) => {
            return Err(format!("Failed ssh connection to {0}. Error: {e}", sys.ip));
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
