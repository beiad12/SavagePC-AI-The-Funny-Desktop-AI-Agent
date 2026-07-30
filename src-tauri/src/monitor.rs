use crate::commands::ProviderConfig;
use crate::llm::{self, LlmProvider};
use crate::memory::Memory;
use crate::personality::{self, Personality};
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
    /// Factual English description of what's happening, fed to the LLM as the prompt
    /// for a fresh AI-generated notification each time.
    situation: String,
    /// Localized canned message used only if no LLM API key is configured yet.
    fallback: String,
}

fn load_provider_config(memory: &Memory) -> Option<ProviderConfig> {
    let raw = memory.get_setting("provider_config").ok().flatten()?;
    serde_json::from_str(&raw).ok()
}

/// Runs for the lifetime of the app process (including while the window is hidden),
/// polling telemetry and raising OS notifications + stored alerts when thresholds
/// are crossed, with a per-alert-type cooldown so it doesn't spam the user. Each
/// notification's wording is generated fresh by the configured LLM (falling back to
/// a canned localized line only if no API key is set yet), so it doesn't repeat.
pub fn spawn(app: AppHandle, telemetry: Arc<Mutex<TelemetryCollector>>, memory: Arc<Memory>) {
    tauri::async_runtime::spawn(async move {
        let mut last_fired: HashMap<&'static str, Instant> = HashMap::new();
        let mut was_charging: Option<bool> = None;

        loop {
            tokio::time::sleep(POLL_INTERVAL).await;

            let snapshot = {
                let mut collector = telemetry.lock().unwrap();
                collector.collect()
            };
            let language = memory.get_setting("language").ok().flatten().unwrap_or_else(|| "en".to_string());
            let personality_code = memory.get_setting("personality").ok().flatten().unwrap_or_else(|| "sarcastic".to_string());
            let personality = Personality::from_str(&personality_code);

            let mut checks: Vec<Check> = Vec::new();

            if snapshot.cpu_usage_percent > 90.0 {
                checks.push(Check {
                    key: "cpu_high",
                    severity: "warning",
                    situation: format!(
                        "Your CPU usage just crossed 90% — currently {:.0}%.",
                        snapshot.cpu_usage_percent
                    ),
                    fallback: personality::alert_cpu_high(&language, snapshot.cpu_usage_percent),
                });
            }

            if snapshot.ram_percent > 90.0 {
                checks.push(Check {
                    key: "ram_high",
                    severity: "warning",
                    situation: format!(
                        "Your RAM usage just crossed 90% — currently {:.0}% ({:.1}/{:.1} GB used).",
                        snapshot.ram_percent, snapshot.ram_used_gb, snapshot.ram_total_gb
                    ),
                    fallback: personality::alert_ram_high(
                        &language,
                        snapshot.ram_percent,
                        snapshot.ram_used_gb,
                        snapshot.ram_total_gb,
                    ),
                });
            }

            for disk in &snapshot.disks {
                if disk.used_percent > 90.0 {
                    checks.push(Check {
                        key: "disk_full",
                        severity: "critical",
                        situation: format!(
                            "Your disk {} just crossed 90% full — currently {:.0}% used, only {:.1} GB free.",
                            disk.name, disk.used_percent, disk.free_gb
                        ),
                        fallback: personality::alert_disk_full(&language, &disk.name, disk.used_percent, disk.free_gb),
                    });
                }
            }

            if let (Some(pct), Some(charging)) = (snapshot.battery_percent, snapshot.battery_charging) {
                if pct < 15.0 && !charging {
                    checks.push(Check {
                        key: "battery_low",
                        severity: "warning",
                        situation: format!(
                            "Your battery just dropped below 15% — currently {pct:.0}% — and you are NOT plugged in to charge."
                        ),
                        fallback: personality::alert_battery_low(&language, pct),
                    });
                }
                // Fire once on the transition into charging, not on every poll while charging.
                if charging && was_charging != Some(true) {
                    checks.push(Check {
                        key: "battery_charging",
                        severity: "info",
                        situation: format!(
                            "The user just plugged you in — you started charging again, currently at {pct:.0}%."
                        ),
                        fallback: personality::alert_battery_charging(&language, pct),
                    });
                }
                was_charging = Some(charging);
            }

            if checks.is_empty() {
                continue;
            }

            let provider_config = load_provider_config(&memory);

            for check in checks {
                let now = Instant::now();
                let on_cooldown = last_fired
                    .get(check.key)
                    .is_some_and(|t| now.duration_since(*t) < ALERT_COOLDOWN);
                if on_cooldown {
                    continue;
                }
                last_fired.insert(check.key, now);

                let message = generate_notification_message(&provider_config, personality, &language, &check)
                    .await
                    .unwrap_or(check.fallback);

                let _ = memory.log_alert(check.severity, &message);
                let _ = app.notification().builder().title("SavagePC AI").body(&message).show();
            }
        }
    });
}

async fn generate_notification_message(
    provider_config: &Option<ProviderConfig>,
    personality: Personality,
    language: &str,
    check: &Check,
) -> Option<String> {
    let config = provider_config.as_ref()?;
    if config.api_key.trim().is_empty() {
        return None;
    }
    let provider = LlmProvider::from_str(&config.provider);
    let system_prompt = personality::build_notification_system_prompt(personality, language);
    let text = llm::generate_short_text(provider, &system_prompt, &check.situation, &config.api_key, &config.model)
        .await
        .ok()?;
    let text = text.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}
