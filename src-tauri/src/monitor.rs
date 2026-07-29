use crate::memory::Memory;
use crate::telemetry::TelemetryCollector;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

const POLL_INTERVAL: Duration = Duration::from_secs(20);
const ALERT_COOLDOWN: Duration = Duration::from_secs(30 * 60);

struct Check {
    key: &'static str,
    severity: &'static str,
    message: String,
}

/// Runs for the lifetime of the app process (including while the window is hidden),
/// polling telemetry and raising OS notifications + stored alerts when thresholds
/// are crossed, with a per-alert-type cooldown so it doesn't spam the user.
pub fn spawn(app: AppHandle, telemetry: Arc<Mutex<TelemetryCollector>>, memory: Arc<Memory>) {
    tauri::async_runtime::spawn(async move {
        let mut last_fired: HashMap<&'static str, Instant> = HashMap::new();

        loop {
            tokio::time::sleep(POLL_INTERVAL).await;

            let snapshot = {
                let mut collector = telemetry.lock().unwrap();
                collector.collect()
            };

            let mut checks: Vec<Check> = Vec::new();

            if snapshot.cpu_usage_percent > 90.0 {
                checks.push(Check {
                    key: "cpu_high",
                    severity: "warning",
                    message: format!(
                        "🔥 CPU is pinned at {:.0}%. Something's cooking.",
                        snapshot.cpu_usage_percent
                    ),
                });
            }

            if snapshot.ram_percent > 90.0 {
                checks.push(Check {
                    key: "ram_high",
                    severity: "warning",
                    message: format!(
                        "🧠 RAM at {:.0}% ({:.1}/{:.1} GB). Time to close some tabs.",
                        snapshot.ram_percent, snapshot.ram_used_gb, snapshot.ram_total_gb
                    ),
                });
            }

            for disk in &snapshot.disks {
                if disk.used_percent > 90.0 {
                    checks.push(Check {
                        key: "disk_full",
                        severity: "critical",
                        message: format!(
                            "💾 {} is {:.0}% full ({:.1} GB free). Time to clean up.",
                            disk.name, disk.used_percent, disk.free_gb
                        ),
                    });
                }
            }

            if let (Some(pct), Some(charging)) = (snapshot.battery_percent, snapshot.battery_charging) {
                if pct < 15.0 && !charging {
                    checks.push(Check {
                        key: "battery_low",
                        severity: "warning",
                        message: format!("🔋 Battery at {pct:.0}% and not charging. Plug in before it's too late."),
                    });
                }
            }

            for check in checks {
                let now = Instant::now();
                let on_cooldown = last_fired
                    .get(check.key)
                    .is_some_and(|t| now.duration_since(*t) < ALERT_COOLDOWN);
                if on_cooldown {
                    continue;
                }
                last_fired.insert(check.key, now);

                let _ = memory.log_alert(check.severity, &check.message);
                let _ = app
                    .notification()
                    .builder()
                    .title("SavagePC AI")
                    .body(&check.message)
                    .show();
            }
        }
    });
}
