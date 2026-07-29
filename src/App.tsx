import { useEffect, useState } from "react";
import Sidebar from "@/components/Sidebar";
import Dashboard from "@/components/Dashboard";
import ChatPanel from "@/components/ChatPanel";
import SettingsPanel from "@/components/SettingsPanel";
import NotificationsPanel from "@/components/NotificationsPanel";
import { getTelemetry, listAlerts } from "@/lib/bridge";
import { useAppStore } from "@/state/store";

export type View = "chat" | "dashboard" | "settings" | "alerts";

export default function App() {
  const [view, setView] = useState<View>("chat");
  const theme = useAppStore((s) => s.theme);
  const setTelemetry = useAppStore((s) => s.setTelemetry);
  const setAlerts = useAppStore((s) => s.setAlerts);

  useEffect(() => {
    document.documentElement.classList.toggle("dark", theme === "dark");
  }, [theme]);

  useEffect(() => {
    let cancelled = false;
    const poll = async () => {
      try {
        const t = await getTelemetry();
        if (!cancelled) setTelemetry(t);
      } catch (err) {
        console.error("telemetry poll failed", err);
      }
    };
    poll();
    const id = setInterval(poll, 3000);
    return () => {
      cancelled = true;
      clearInterval(id);
    };
  }, [setTelemetry]);

  useEffect(() => {
    let cancelled = false;
    const poll = async () => {
      try {
        const a = await listAlerts();
        if (!cancelled) setAlerts(a);
      } catch (err) {
        console.error("alerts poll failed", err);
      }
    };
    poll();
    const id = setInterval(poll, 10000);
    return () => {
      cancelled = true;
      clearInterval(id);
    };
  }, [setAlerts]);

  return (
    <div className="flex h-screen w-screen overflow-hidden bg-savage-bg text-slate-100">
      <Sidebar view={view} onChange={setView} />
      <main className="flex-1 overflow-hidden">
        {view === "chat" && <ChatPanel />}
        {view === "dashboard" && <Dashboard />}
        {view === "settings" && <SettingsPanel />}
        {view === "alerts" && <NotificationsPanel />}
      </main>
    </div>
  );
}
