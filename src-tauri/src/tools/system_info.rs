use anyhow::Result;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};

pub fn analyze_large_folders() -> Result<String> {
    let Some(home) = dirs::home_dir() else {
        return Ok("Couldn't locate home directory.".to_string());
    };

    let mut entries: Vec<(String, u64)> = Vec::new();
    if let Ok(read) = fs::read_dir(&home) {
        for entry in read.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let size = shallow_dir_size(&path, 2);
                entries.push((path.display().to_string(), size));
            }
        }
    }
    entries.sort_by(|a, b| b.1.cmp(&a.1));
    entries.truncate(8);

    if entries.is_empty() {
        return Ok("No accessible folders found under the home directory.".to_string());
    }

    let lines: Vec<String> = entries
        .into_iter()
        .map(|(name, size)| format!("{name}: {:.2} GB", size as f64 / (1024.0 * 1024.0 * 1024.0)))
        .collect();
    Ok(lines.join("\n"))
}

#[derive(Clone, Serialize)]
struct ScanProgress {
    scanned: u64,
    current_path: String,
}

#[derive(Clone, Serialize)]
struct ScanComplete {
    scanned: u64,
}

const SCAN_TIME_BUDGET: Duration = Duration::from_secs(20);
const SCAN_EMIT_INTERVAL: Duration = Duration::from_millis(200);
const SKIP_DIR_NAMES: [&str; 5] = ["node_modules", ".git", ".cache", "AppData", "Library"];

/// Deep walk of the user's home directory looking for the individual largest files
/// (not just folder totals), emitting `scan_progress` events as it goes so the UI can
/// show a live scanning animation, and a final `scan_complete` event.
pub fn find_large_files(app: &AppHandle) -> Result<String> {
    let Some(home) = dirs::home_dir() else {
        return Ok("Couldn't locate home directory.".to_string());
    };

    let start = Instant::now();
    let mut last_emit = Instant::now();
    let mut scanned: u64 = 0;
    let mut largest: Vec<(PathBuf, u64)> = Vec::new();
    let mut stack: Vec<PathBuf> = vec![home];

    while let Some(dir) = stack.pop() {
        if start.elapsed() > SCAN_TIME_BUDGET {
            break;
        }
        let Ok(read) = fs::read_dir(&dir) else { continue };
        for entry in read.flatten() {
            let path = entry.path();
            let Ok(meta) = entry.metadata() else { continue };
            scanned += 1;

            if meta.is_dir() {
                let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if !SKIP_DIR_NAMES.contains(&name) {
                    stack.push(path.clone());
                }
            } else if meta.is_file() {
                largest.push((path.clone(), meta.len()));
            }

            if last_emit.elapsed() > SCAN_EMIT_INTERVAL {
                let _ = app.emit(
                    "scan_progress",
                    ScanProgress { scanned, current_path: path.display().to_string() },
                );
                last_emit = Instant::now();
            }
        }
    }

    let _ = app.emit("scan_complete", ScanComplete { scanned });

    largest.sort_by(|a, b| b.1.cmp(&a.1));
    largest.truncate(15);

    if largest.is_empty() {
        return Ok(format!("Scanned {scanned} items but found no accessible files."));
    }

    let lines: Vec<String> = largest
        .into_iter()
        .map(|(path, size)| format!("{:.2} GB — {}", size as f64 / (1024.0 * 1024.0 * 1024.0), path.display()))
        .collect();
    Ok(format!(
        "Scanned {scanned} files/folders under the home directory. Largest individual files found:\n{}",
        lines.join("\n")
    ))
}

/// Bounded-depth size walk so this stays fast on huge home directories.
fn shallow_dir_size(path: &Path, depth: u8) -> u64 {
    let mut total = 0u64;
    let Ok(read) = fs::read_dir(path) else {
        return 0;
    };
    for entry in read.flatten() {
        let p = entry.path();
        if let Ok(meta) = entry.metadata() {
            if meta.is_file() {
                total += meta.len();
            } else if meta.is_dir() && depth > 0 {
                total += shallow_dir_size(&p, depth - 1);
            }
        }
    }
    total
}
