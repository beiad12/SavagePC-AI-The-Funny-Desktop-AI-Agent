import { useEffect, useRef, useState } from "react";
import { Area, AreaChart, ResponsiveContainer, Tooltip, XAxis, YAxis } from "recharts";
import StatCard from "@/components/StatCard";
import { runTool } from "@/lib/bridge";
import { useAppStore } from "@/state/store";

const MAINTENANCE_ACTIONS = [
  { tool: "empty_recycle_bin", label: "Empty Recycle Bin", icon: "🗑️" },
  { tool: "delete_temp_files", label: "Delete Temp Files", icon: "🧹" },
  { tool: "clear_browser_cache", label: "Clear Browser Cache", icon: "🌐" },
  { tool: "open_task_manager", label: "Open Task Manager", icon: "📋" },
];

export default function Dashboard() {
  const telemetry = useAppStore((s) => s.telemetry);
  const [history, setHistory] = useState<{ t: number; cpu: number; ram: number }[]>([]);
  const [actionResult, setActionResult] = useState<string | null>(null);
  const [busy, setBusy] = useState<string | null>(null);
  const lastTimestamp = useRef<number>(0);

  useEffect(() => {
    if (!telemetry || telemetry.timestamp === lastTimestamp.current) return;
    lastTimestamp.current = telemetry.timestamp;
    setHistory((prev) => {
      const next = [...prev, { t: telemetry.timestamp, cpu: telemetry.cpu_usage_percent, ram: telemetry.ram_percent }];
      return next.slice(-40);
    });
  }, [telemetry]);

  const runAction = async (tool: string) => {
    setBusy(tool);
    setActionResult(null);
    try {
      const result = await runTool(tool);
      setActionResult(result);
    } catch (err) {
      setActionResult(`Failed: ${String(err)}`);
    } finally {
      setBusy(null);
    }
  };

  if (!telemetry) {
    return <div className="flex h-full items-center justify-center text-slate-400">Loading telemetry…</div>;
  }

  const uptimeHours = Math.floor(telemetry.uptime_seconds / 3600);

  return (
    <div className="h-full overflow-y-auto p-6">
      <h1 className="mb-4 text-xl font-semibold">System Dashboard</h1>

      <div className="grid grid-cols-2 gap-4 lg:grid-cols-4">
        <StatCard
          label="CPU"
          value={`${Math.round(telemetry.cpu_usage_percent)}%`}
          sub={`${telemetry.cpu_name} · ${telemetry.cpu_cores} cores`}
          percent={telemetry.cpu_usage_percent}
          danger={telemetry.cpu_usage_percent > 85}
        />
        <StatCard
          label="RAM"
          value={`${telemetry.ram_used_gb.toFixed(1)} / ${telemetry.ram_total_gb.toFixed(0)} GB`}
          sub={`${Math.round(telemetry.ram_percent)}% used`}
          percent={telemetry.ram_percent}
          danger={telemetry.ram_percent > 85}
        />
        <StatCard
          label="Network"
          value={`↓ ${(telemetry.network_rx_kbps / 1024).toFixed(1)} MB/s`}
          sub={`↑ ${(telemetry.network_tx_kbps / 1024).toFixed(1)} MB/s`}
        />
        <StatCard
          label="Uptime"
          value={`${uptimeHours}h`}
          sub={telemetry.battery_percent !== null ? `Battery ${telemetry.battery_percent}%` : "Desktop / no battery"}
        />
      </div>

      <div className="mt-4 grid grid-cols-1 gap-4 lg:grid-cols-2">
        {telemetry.disks.map((disk) => (
          <StatCard
            key={disk.mount_point}
            label={`Disk ${disk.name}`}
            value={`${disk.free_gb.toFixed(0)} GB free`}
            sub={`${disk.total_gb.toFixed(0)} GB total · ${disk.used_percent.toFixed(0)}% used`}
            percent={disk.used_percent}
            danger={disk.used_percent > 90}
          />
        ))}
      </div>

      <div className="glass mt-4 rounded-2xl p-4">
        <div className="mb-2 text-xs uppercase tracking-wide text-slate-400">CPU / RAM history</div>
        <div className="h-40">
          <ResponsiveContainer width="100%" height="100%">
            <AreaChart data={history}>
              <defs>
                <linearGradient id="cpuGrad" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="0%" stopColor="#ff4d5e" stopOpacity={0.5} />
                  <stop offset="100%" stopColor="#ff4d5e" stopOpacity={0} />
                </linearGradient>
                <linearGradient id="ramGrad" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="0%" stopColor="#7c5cff" stopOpacity={0.5} />
                  <stop offset="100%" stopColor="#7c5cff" stopOpacity={0} />
                </linearGradient>
              </defs>
              <XAxis dataKey="t" hide />
              <YAxis domain={[0, 100]} hide />
              <Tooltip
                contentStyle={{ background: "#12151c", border: "1px solid rgba(255,255,255,0.1)" }}
                labelFormatter={() => ""}
              />
              <Area type="monotone" dataKey="cpu" stroke="#ff4d5e" fill="url(#cpuGrad)" strokeWidth={2} name="CPU %" />
              <Area type="monotone" dataKey="ram" stroke="#7c5cff" fill="url(#ramGrad)" strokeWidth={2} name="RAM %" />
            </AreaChart>
          </ResponsiveContainer>
        </div>
      </div>

      <div className="mt-4 grid grid-cols-1 gap-4 lg:grid-cols-2">
        <div className="glass rounded-2xl p-4">
          <div className="mb-2 text-xs uppercase tracking-wide text-slate-400">Top processes</div>
          <ul className="space-y-1 text-sm">
            {telemetry.top_processes.map((p) => (
              <li key={p.pid} className="flex justify-between text-slate-300">
                <span>{p.name}</span>
                <span className="text-slate-500">
                  {p.cpu_percent.toFixed(1)}% CPU · {p.ram_mb.toFixed(0)} MB
                </span>
              </li>
            ))}
          </ul>
        </div>

        <div className="glass rounded-2xl p-4">
          <div className="mb-2 text-xs uppercase tracking-wide text-slate-400">One-click maintenance</div>
          <div className="grid grid-cols-2 gap-2">
            {MAINTENANCE_ACTIONS.map((action) => (
              <button
                key={action.tool}
                onClick={() => runAction(action.tool)}
                disabled={busy !== null}
                className="flex items-center gap-2 rounded-xl bg-white/5 px-3 py-2 text-sm transition hover:bg-white/10 disabled:opacity-50"
              >
                <span>{action.icon}</span>
                <span>{busy === action.tool ? "Running…" : action.label}</span>
              </button>
            ))}
          </div>
          {actionResult && <div className="mt-3 rounded-lg bg-black/30 p-2 text-xs text-slate-300">{actionResult}</div>}
        </div>
      </div>
    </div>
  );
}
