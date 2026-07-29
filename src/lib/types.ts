export type Personality = "friendly" | "sarcastic" | "savage";

export type LlmProvider = "openai" | "mistral" | "gemini" | "grok";

export interface ProviderConfig {
  provider: LlmProvider;
  apiKey: string;
  model: string;
}

export interface Telemetry {
  cpu_usage_percent: number;
  cpu_name: string;
  cpu_cores: number;
  ram_total_gb: number;
  ram_used_gb: number;
  ram_percent: number;
  disks: DiskInfo[];
  network_rx_kbps: number;
  network_tx_kbps: number;
  battery_percent: number | null;
  battery_charging: boolean | null;
  uptime_seconds: number;
  process_count: number;
  top_processes: ProcessInfo[];
  timestamp: number;
}

export interface DiskInfo {
  name: string;
  mount_point: string;
  total_gb: number;
  free_gb: number;
  used_percent: number;
}

export interface ProcessInfo {
  pid: number;
  name: string;
  cpu_percent: number;
  ram_mb: number;
}

export interface ChatMessage {
  id: string;
  role: "user" | "assistant" | "system" | "tool";
  content: string;
  createdAt: number;
  toolCalls?: ToolCallRecord[];
}

export interface ToolCallRecord {
  name: string;
  args: Record<string, unknown>;
  result?: string;
}

export interface ToolDefinition {
  name: string;
  description: string;
  category: string;
  dangerous: boolean;
}

export interface Alert {
  id: number;
  severity: "warning" | "critical" | string;
  message: string;
  created_at: number;
  acknowledged: boolean;
}
