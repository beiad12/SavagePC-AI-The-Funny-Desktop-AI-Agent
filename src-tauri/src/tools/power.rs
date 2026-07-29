use anyhow::Result;
use std::process::Command;

pub fn open_task_manager() -> Result<String> {
    #[cfg(target_os = "windows")]
    Command::new("taskmgr").spawn()?;
    #[cfg(target_os = "macos")]
    Command::new("open")
        .args(["-a", "Activity Monitor"])
        .spawn()?;
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        for candidate in ["gnome-system-monitor", "ksysguard", "xterm -e top"] {
            let mut parts = candidate.split(' ');
            if let Some(bin) = parts.next() {
                if Command::new(bin).args(parts).spawn().is_ok() {
                    break;
                }
            }
        }
    }
    Ok("Task manager launched.".to_string())
}

pub fn open_settings() -> Result<String> {
    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(["/C", "start", "ms-settings:"])
        .spawn()?;
    #[cfg(target_os = "macos")]
    Command::new("open")
        .args(["-b", "com.apple.systempreferences"])
        .spawn()?;
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    Command::new("gnome-control-center").spawn().ok();
    Ok("Settings opened.".to_string())
}

pub fn kill_process(process_name: &str) -> Result<String> {
    #[cfg(target_os = "windows")]
    let status = Command::new("taskkill")
        .args(["/IM", process_name, "/F"])
        .status()?;
    #[cfg(not(target_os = "windows"))]
    let status = Command::new("pkill").arg(process_name).status()?;

    Ok(if status.success() {
        format!("Killed process(es) matching '{process_name}'.")
    } else {
        format!("No running process matched '{process_name}'.")
    })
}

pub fn launch_application(path: &str) -> Result<String> {
    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(["/C", "start", "", path])
        .spawn()?;
    #[cfg(target_os = "macos")]
    Command::new("open").arg(path).spawn()?;
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    Command::new(path).spawn()?;
    Ok(format!("Launched '{path}'."))
}

pub fn restart_pc() -> Result<String> {
    #[cfg(target_os = "windows")]
    Command::new("shutdown").args(["/r", "/t", "5"]).spawn()?;
    #[cfg(target_os = "macos")]
    Command::new("osascript")
        .args(["-e", "tell application \"System Events\" to restart"])
        .spawn()?;
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    Command::new("systemctl").arg("reboot").spawn()?;
    Ok("Restart initiated.".to_string())
}

pub fn shutdown_pc() -> Result<String> {
    #[cfg(target_os = "windows")]
    Command::new("shutdown").args(["/s", "/t", "5"]).spawn()?;
    #[cfg(target_os = "macos")]
    Command::new("osascript")
        .args(["-e", "tell application \"System Events\" to shut down"])
        .spawn()?;
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    Command::new("systemctl").arg("poweroff").spawn()?;
    Ok("Shutdown initiated.".to_string())
}

pub fn sleep_pc() -> Result<String> {
    #[cfg(target_os = "windows")]
    Command::new("rundll32.exe")
        .args(["powrprof.dll,SetSuspendState", "0,1,0"])
        .spawn()?;
    #[cfg(target_os = "macos")]
    Command::new("pmset").arg("sleepnow").spawn()?;
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    Command::new("systemctl").arg("suspend").spawn()?;
    Ok("Sleep requested.".to_string())
}
