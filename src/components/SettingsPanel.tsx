import { useEffect, useState } from "react";
import { loadProviderConfig, saveProviderConfig } from "@/lib/bridge";
import { useAppStore } from "@/state/store";
import type { LlmProvider } from "@/lib/types";

const PROVIDERS: { id: LlmProvider; label: string }[] = [
  { id: "openai", label: "OpenAI" },
  { id: "mistral", label: "Mistral AI" },
  { id: "gemini", label: "Google Gemini" },
  { id: "grok", label: "xAI Grok" },
];

export default function SettingsPanel() {
  const provider = useAppStore((s) => s.provider);
  const setProvider = useAppStore((s) => s.setProvider);
  const theme = useAppStore((s) => s.theme);
  const toggleTheme = useAppStore((s) => s.toggleTheme);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    loadProviderConfig().then((cfg) => {
      if (cfg) setProvider(cfg);
    });
  }, [setProvider]);

  const save = async () => {
    await saveProviderConfig(provider);
    setSaved(true);
    setTimeout(() => setSaved(false), 1500);
  };

  return (
    <div className="h-full overflow-y-auto p-6">
      <h1 className="mb-4 text-xl font-semibold">Settings</h1>

      <section className="glass mb-4 rounded-2xl p-4">
        <h2 className="mb-3 text-sm font-semibold text-slate-300">AI Provider</h2>
        <div className="mb-3 grid grid-cols-2 gap-2 lg:grid-cols-4">
          {PROVIDERS.map((p) => (
            <button
              key={p.id}
              onClick={() => setProvider({ provider: p.id })}
              className={`rounded-xl px-3 py-2 text-sm transition ${
                provider.provider === p.id ? "bg-white/15 text-white" : "bg-white/5 text-slate-400 hover:bg-white/10"
              }`}
            >
              {p.label}
            </button>
          ))}
        </div>

        <label className="mb-1 block text-xs text-slate-400">API Key</label>
        <input
          type="password"
          value={provider.apiKey}
          onChange={(e) => setProvider({ apiKey: e.target.value })}
          placeholder="sk-…"
          className="mb-3 w-full rounded-xl bg-white/5 px-4 py-2 text-sm outline-none focus:bg-white/10"
        />

        <label className="mb-1 block text-xs text-slate-400">Model</label>
        <input
          value={provider.model}
          onChange={(e) => setProvider({ model: e.target.value })}
          className="mb-3 w-full rounded-xl bg-white/5 px-4 py-2 text-sm outline-none focus:bg-white/10"
        />

        <button
          onClick={save}
          className="rounded-xl bg-gradient-to-br from-savage-accent2 to-savage-accent px-4 py-2 text-sm font-medium"
        >
          {saved ? "Saved ✓" : "Save"}
        </button>
        <p className="mt-2 text-xs text-slate-500">
          Keys are encrypted at rest and only sent directly to the provider you choose. No telemetry leaves this
          device unless it's part of a message you send to the AI.
        </p>
      </section>

      <section className="glass rounded-2xl p-4">
        <h2 className="mb-3 text-sm font-semibold text-slate-300">Appearance</h2>
        <button onClick={toggleTheme} className="rounded-xl bg-white/5 px-4 py-2 text-sm hover:bg-white/10">
          Switch to {theme === "dark" ? "light" : "dark"} mode
        </button>
      </section>
    </div>
  );
}
