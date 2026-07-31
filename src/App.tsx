import { useEffect, useState } from "react";
import Sidebar from "@/components/Sidebar";
import Dashboard from "@/components/Dashboard";
import ChatPanel from "@/components/ChatPanel";
import SettingsPanel from "@/components/SettingsPanel";
import NotificationsPanel from "@/components/NotificationsPanel";
import OnboardingWizard from "@/components/OnboardingWizard";
import { getTelemetry, isTauri, listAlerts, loadLanguage, loadPersonality, loadProviderConfig } from "@/lib/bridge";
import { useAppStore } from "@/state/store";
import { LANGUAGES, type LanguageCode } from "@/lib/i18n";
import type { Personality } from "@/lib/types";

export type View = "chat" | "dashboard" | "settings" | "alerts";

export default function App() {
  const [view, setView] = useState<View>("chat");
  const [needsOnboarding, setNeedsOnboarding] = useState<boolean | null>(null);
  const theme = useAppStore((s) => s.theme);
  const language = useAppStore((s) => s.language);
  const setLanguage = useAppStore((s) => s.setLanguage);
  const setPersonality = useAppStore((s) => s.setPersonality);
  const setTelemetry = useAppStore((s) => s.setTelemetry);
  const setAlerts = useAppStore((s) => s.setAlerts);

  useEffect(() => {
    document.documentElement.classList.toggle("dark", theme === "dark");
  }, [theme]);

  useEffect(() => {
    loadLanguage().then((saved) => {
      if (saved) setLanguage(saved as LanguageCode);
    });
    loadPersonality().then((saved) => {
      if (saved) setPersonality(saved as Personality);
    });
    if (!isTauri) {
      setNeedsOnboarding(false);
      return;
    }
    loadProviderConfig().then((cfg) => {
      setNeedsOnboarding(!cfg || !cfg.apiKey);
    });
  }, [setLanguage, setPersonality]);

  useEffect(() => {
    const meta = LANGUAGES.find((l) => l.code === language);
    document.documentElement.lang = language;
    document.documentElement.dir = meta?.rtl ? "rtl" : "ltr";
  }, [language]);

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

  if (needsOnboarding === null) {
    return <div className="h-screen w-screen bg-savage-bg" />;
  }

  if (needsOnboarding) {
    return (
      <div className="h-screen w-screen bg-savage-bg text-slate-100">
        <OnboardingWizard onComplete={() => setNeedsOnboarding(false)} />
      </div>
    );
  }

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
