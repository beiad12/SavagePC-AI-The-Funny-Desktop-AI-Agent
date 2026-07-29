use anyhow::Result;

pub fn empty_recycle_bin() -> Result<String> {
    #[cfg(target_os = "windows")]
    {
        // SHEmptyRecycleBinW via a PowerShell fallback keeps this dependency-free.
        let status = std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "Clear-RecycleBin -Force -ErrorAction SilentlyContinue",
            ])
            .status();
        return Ok(match status {
            Ok(s) if s.success() => "Recycle bin emptied.".to_string(),
            _ => "Attempted to empty recycle bin (some items may be locked).".to_string(),
        });
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("osascript")
            .args(["-e", "tell application \"Finder\" to empty trash"])
            .status();
        return Ok("Trash emptied.".to_string());
    }
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        if let Some(home) = dirs::home_dir() {
            let trash = home.join(".local/share/Trash/files");
            if trash.exists() {
                let _ = std::fs::remove_dir_all(&trash);
                let _ = std::fs::create_dir_all(&trash);
            }
        }
        Ok("Trash emptied.".to_string())
    }
}

pub fn delete_temp_files() -> Result<String> {
    let temp_dir = std::env::temp_dir();
    let mut freed_bytes: u64 = 0;
    let mut deleted = 0u32;

    if let Ok(entries) = std::fs::read_dir(&temp_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let size = dir_size(&path).unwrap_or(0);
            let removed = if path.is_dir() {
                std::fs::remove_dir_all(&path).is_ok()
            } else {
                std::fs::remove_file(&path).is_ok()
            };
            if removed {
                freed_bytes += size;
                deleted += 1;
            }
        }
    }

    Ok(format!(
        "Deleted {deleted} temp items, freed {:.2} MB from {}.",
        freed_bytes as f64 / (1024.0 * 1024.0),
        temp_dir.display()
    ))
}

pub fn clear_browser_cache() -> Result<String> {
    let Some(home) = dirs::home_dir() else {
        return Ok("Couldn't locate home directory.".to_string());
    };

    let candidates = [
        "AppData/Local/Google/Chrome/User Data/Default/Cache",
        "AppData/Local/Microsoft/Edge/User Data/Default/Cache",
        "Library/Caches/Google/Chrome/Default/Cache",
        ".cache/google-chrome/Default/Cache",
        ".cache/mozilla/firefox",
    ];

    let mut cleared = 0u32;
    for rel in candidates {
        let path = home.join(rel);
        if path.exists() && std::fs::remove_dir_all(&path).is_ok() {
            let _ = std::fs::create_dir_all(&path);
            cleared += 1;
        }
    }

    Ok(if cleared > 0 {
        format!("Cleared cache for {cleared} browser profile(s).")
    } else {
        "No known browser cache directories were found on this system.".to_string()
    })
}

fn dir_size(path: &std::path::Path) -> std::io::Result<u64> {
    if path.is_file() {
        return Ok(path.metadata()?.len());
    }
    let mut total = 0u64;
    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            total += dir_size(&entry.path()).unwrap_or(0);
        }
    }
    Ok(total)
}
