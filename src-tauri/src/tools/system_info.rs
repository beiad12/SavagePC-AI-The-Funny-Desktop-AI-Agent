use anyhow::Result;
use std::fs;
use std::path::Path;

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
