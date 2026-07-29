import { invoke } from "@tauri-apps/api/core";
import type { Alert, ChatMessage, ProviderConfig, Telemetry } from "./types";

const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

function mockTelemetry(): Telemetry {
  return {
    cpu_usage_percent: 20 + Math.random() * 40,
    cpu_name: "Mock CPU (browser preview)",
    cpu_cores: 8,
    ram_total_gb: 32,
    ram_used_gb: 14 + Math.random() * 4,
    ram_percent: 45 + Math.random() * 15,
    disks: [
      { name: "C:", mount_point: "C:\\", total_gb: 512, free_gb: 38, used_percent: 92.6 },
    ],
    network_rx_kbps: Math.random() * 2000,
    network_tx_kbps: Math.random() * 500,
    battery_percent: 71,
    battery_charging: false,
    uptime_seconds: 3600 * 19,
    process_count: 214,
    top_processes: [
      { pid: 1234, name: "chrome.exe", cpu_percent: 12.3, ram_mb: 3200 },
      { pid: 5678, name: "Discord.exe", cpu_percent: 4.1, ram_mb: 900 },
    ],
    timestamp: Date.now(),
  };
}

export async function getTelemetry(): Promise<Telemetry> {
  if (!isTauri) return mockTelemetry();
  return invoke<Telemetry>("get_telemetry");
}

export async function runTool(name: string, args: Record<string, unknown> = {}): Promise<string> {
  if (!isTauri) return `[preview mode] would run tool "${name}" with ${JSON.stringify(args)}`;
  return invoke<string>("run_tool", { name, args });
}

export async function listTools() {
  if (!isTauri) return [];
  return invoke("list_tools");
}

export async function sendChatMessage(
  history: ChatMessage[],
  personality: string,
  provider: ProviderConfig,
): Promise<ChatMessage> {
  if (!isTauri) {
    return {
      id: crypto.randomUUID(),
      role: "assistant",
      content:
        "I'm running in browser preview mode, so I can't reach a real LLM or your real system telemetry. Run this inside the Tauri desktop shell for the full experience.",
      createdAt: Date.now(),
    };
  }
  return invoke<ChatMessage>("send_chat_message", { history, personality, provider });
}

export async function saveProviderConfig(config: ProviderConfig): Promise<void> {
  if (!isTauri) return;
  await invoke("save_provider_config", { config });
}

export async function loadProviderConfig(): Promise<ProviderConfig | null> {
  if (!isTauri) return null;
  return invoke<ProviderConfig | null>("load_provider_config");
}

export async function listAlerts(): Promise<Alert[]> {
  if (!isTauri) return [];
  return invoke<Alert[]>("list_alerts");
}

export async function acknowledgeAlert(id: number): Promise<void> {
  if (!isTauri) return;
  await invoke("acknowledge_alert", { id });
}
