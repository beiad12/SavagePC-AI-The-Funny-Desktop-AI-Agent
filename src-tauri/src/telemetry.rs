use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::{Disks, Networks, System};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_gb: f64,
    pub free_gb: f64,
    pub used_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub ram_mb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Telemetry {
    pub cpu_usage_percent: f32,
    pub cpu_name: String,
    pub cpu_cores: usize,
    pub ram_total_gb: f64,
    pub ram_used_gb: f64,
    pub ram_percent: f64,
    pub disks: Vec<DiskInfo>,
    pub network_rx_kbps: f64,
    pub network_tx_kbps: f64,
    pub battery_percent: Option<f32>,
    pub battery_charging: Option<bool>,
    pub uptime_seconds: u64,
    pub process_count: usize,
    pub top_processes: Vec<ProcessInfo>,
    pub timestamp: u64,
}

const BYTES_PER_GB: f64 = 1024.0 * 1024.0 * 1024.0;

pub struct TelemetryCollector {
    sys: System,
    networks: Networks,
}

impl TelemetryCollector {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self {
            sys,
            networks: Networks::new_with_refreshed_list(),
        }
    }

    pub fn collect(&mut self) -> Telemetry {
        self.sys.refresh_cpu_usage();
        self.sys.refresh_memory();
        self.sys
            .refresh_processes(sysinfo::ProcessesToUpdate::All, true);
        self.networks.refresh();

        let cpu_usage_percent = self.sys.global_cpu_usage();
        let cpu_name = self
            .sys
            .cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());
        let cpu_cores = self.sys.cpus().len();

        let ram_total_gb = self.sys.total_memory() as f64 / BYTES_PER_GB;
        let ram_used_gb = self.sys.used_memory() as f64 / BYTES_PER_GB;
        let ram_percent = if ram_total_gb > 0.0 {
            (ram_used_gb / ram_total_gb) * 100.0
        } else {
            0.0
        };

        let disks = Disks::new_with_refreshed_list()
            .iter()
            .map(|d| {
                let total_gb = d.total_space() as f64 / BYTES_PER_GB;
                let free_gb = d.available_space() as f64 / BYTES_PER_GB;
                let used_percent = if total_gb > 0.0 {
                    ((total_gb - free_gb) / total_gb) * 100.0
                } else {
                    0.0
                };
                DiskInfo {
                    name: d.name().to_string_lossy().to_string(),
                    mount_point: d.mount_point().to_string_lossy().to_string(),
                    total_gb,
                    free_gb,
                    used_percent,
                }
            })
            .collect();

        let (mut rx, mut tx) = (0u64, 0u64);
        for (_iface, data) in self.networks.iter() {
            rx += data.received();
            tx += data.transmitted();
        }
        let network_rx_kbps = rx as f64 / 1024.0;
        let network_tx_kbps = tx as f64 / 1024.0;

        let mut processes: Vec<ProcessInfo> = self
            .sys
            .processes()
            .values()
            .map(|p| ProcessInfo {
                pid: p.pid().as_u32(),
                name: p.name().to_string_lossy().to_string(),
                cpu_percent: p.cpu_usage(),
                ram_mb: p.memory() as f64 / (1024.0 * 1024.0),
            })
            .collect();
        processes.sort_by(|a, b| {
            b.ram_mb
                .partial_cmp(&a.ram_mb)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let process_count = processes.len();
        let top_processes = processes.into_iter().take(6).collect();

        let uptime_seconds = System::uptime();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let (battery_percent, battery_charging) = read_battery();

        Telemetry {
            cpu_usage_percent,
            cpu_name,
            cpu_cores,
            ram_total_gb,
            ram_used_gb,
            ram_percent,
            disks,
            network_rx_kbps,
            network_tx_kbps,
            battery_percent,
            battery_charging,
            uptime_seconds,
            process_count,
            top_processes,
            timestamp,
        }
    }
}

fn read_battery() -> (Option<f32>, Option<bool>) {
    let manager = match starship_battery::Manager::new() {
        Ok(m) => m,
        Err(_) => return (None, None),
    };
    let mut batteries = match manager.batteries() {
        Ok(b) => b,
        Err(_) => return (None, None),
    };
    match batteries.next() {
        Some(Ok(battery)) => {
            let percent = battery.state_of_charge().value * 100.0;
            let charging = matches!(battery.state(), starship_battery::State::Charging);
            (Some(percent), Some(charging))
        }
        _ => (None, None),
    }
}

impl Default for TelemetryCollector {
    fn default() -> Self {
        Self::new()
    }
}
