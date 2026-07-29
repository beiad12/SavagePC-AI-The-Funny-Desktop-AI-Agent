import { motion } from "framer-motion";
import type { View } from "@/App";
import { useAppStore } from "@/state/store";

const NAV: { id: View; label: string; icon: string }[] = [
  { id: "chat", label: "Chat", icon: "💬" },
  { id: "dashboard", label: "Dashboard", icon: "📊" },
  { id: "settings", label: "Settings", icon: "⚙️" },
];

export default function Sidebar({ view, onChange }: { view: View; onChange: (v: View) => void }) {
  const telemetry = useAppStore((s) => s.telemetry);

  return (
    <aside className="glass flex w-20 flex-col items-center gap-2 border-r border-white/5 py-4">
      <div className="mb-4 flex h-10 w-10 items-center justify-center rounded-xl bg-gradient-to-br from-savage-accent to-savage-accent2 text-lg font-black">
        S
      </div>
      {NAV.map((item) => (
        <button
          key={item.id}
          onClick={() => onChange(item.id)}
          className={`relative flex h-12 w-12 flex-col items-center justify-center rounded-xl text-lg transition ${
            view === item.id ? "bg-white/10 text-white" : "text-slate-400 hover:bg-white/5 hover:text-slate-200"
          }`}
          title={item.label}
        >
          {view === item.id && (
            <motion.div layoutId="nav-highlight" className="absolute inset-0 rounded-xl bg-white/10" />
          )}
          <span className="relative">{item.icon}</span>
        </button>
      ))}
      <div className="mt-auto flex flex-col items-center gap-1 text-[10px] text-slate-500">
        {telemetry ? (
          <>
            <span className={telemetry.cpu_usage_percent > 85 ? "text-savage-accent" : "text-savage-good"}>
              CPU {Math.round(telemetry.cpu_usage_percent)}%
            </span>
            <span className={telemetry.ram_percent > 85 ? "text-savage-accent" : "text-slate-400"}>
              RAM {Math.round(telemetry.ram_percent)}%
            </span>
          </>
        ) : (
          <span>…</span>
        )}
      </div>
    </aside>
  );
}
