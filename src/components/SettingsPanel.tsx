import { useEffect, useState } from "react";
import { loadProviderConfig, saveProviderConfig } from "@/lib/bridge";
import { useAppStore, useT } from "@/state/store";
import { MODEL_OPTIONS } from "@/lib/models";
import { PROVIDERS } from "@/lib/providers";
import LanguageSelector from "@/components/LanguageSelector";

export default function SettingsPanel() {
  const provider = useAppStore((s) => s.provider);
  const setProvider = useAppStore((s) => s.setProvider);
  const theme = useAppStore((s) => s.theme);
  const toggleTheme = useAppStore((s) => s.toggleTheme);
  const [saved, setSaved] = useState(false);
  const t = useT();

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
      <h1 className="mb-4 text-xl font-semibold">{t("settings.title")}</h1>

      <section className="glass mb-4 rounded-2xl p-4">
        <h2 className="mb-3 text-sm font-semibold text-slate-300">{t("settings.language")}</h2>
        <LanguageSelector />
      </section>

      <section className="glass mb-4 rounded-2xl p-4">
        <h2 className="mb-3 text-sm font-semibold text-slate-300">{t("settings.aiProvider")}</h2>
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

        <label className="mb-1 block text-xs text-slate-400">{t("settings.apiKey")}</label>
        <input
          type="password"
          value={provider.apiKey}
          onChange={(e) => setProvider({ apiKey: e.target.value })}
          placeholder="sk-…"
          className="mb-3 w-full rounded-xl bg-white/5 px-4 py-2 text-sm outline-none focus:bg-white/10"
        />

        <label className="mb-1 block text-xs text-slate-400">{t("settings.model")}</label>

        <div className="mb-2">
          <div className="mb-1 text-[10px] uppercase tracking-wide text-slate-500">{t("settings.modelLarge")}</div>
          <div className="flex flex-wrap gap-1.5">
            {MODEL_OPTIONS[provider.provider]
              .filter((m) => m.tier === "large")
              .map((m) => (
                <button
                  key={m.id}
                  onClick={() => setProvider({ model: m.id })}
                  className={`rounded-lg px-2.5 py-1 text-xs transition ${
                    provider.model === m.id ? "bg-white/15 text-white" : "bg-white/5 text-slate-400 hover:bg-white/10"
                  }`}
                >
                  {m.label}
                </button>
              ))}
          </div>
        </div>

        <div className="mb-3">
          <div className="mb-1 text-[10px] uppercase tracking-wide text-slate-500">{t("settings.modelSmall")}</div>
          <div className="flex flex-wrap gap-1.5">
            {MODEL_OPTIONS[provider.provider]
              .filter((m) => m.tier === "small")
              .map((m) => (
                <button
                  key={m.id}
                  onClick={() => setProvider({ model: m.id })}
                  className={`rounded-lg px-2.5 py-1 text-xs transition ${
                    provider.model === m.id ? "bg-white/15 text-white" : "bg-white/5 text-slate-400 hover:bg-white/10"
                  }`}
                >
                  {m.label}
                </button>
              ))}
          </div>
        </div>

        <label className="mb-1 block text-xs text-slate-500">{t("settings.customModel")}</label>
        <input
          value={provider.model}
          onChange={(e) => setProvider({ model: e.target.value })}
          className="mb-3 w-full rounded-xl bg-white/5 px-4 py-2 text-sm outline-none focus:bg-white/10"
        />

        <button
          onClick={save}
          className="rounded-xl bg-gradient-to-br from-savage-accent2 to-savage-accent px-4 py-2 text-sm font-medium"
        >
          {saved ? t("settings.saved") : t("settings.save")}
        </button>
        <p className="mt-2 text-xs text-slate-500">{t("settings.privacyNote")}</p>
      </section>

      <section className="glass rounded-2xl p-4">
        <h2 className="mb-3 text-sm font-semibold text-slate-300">{t("settings.appearance")}</h2>
        <button onClick={toggleTheme} className="rounded-xl bg-white/5 px-4 py-2 text-sm hover:bg-white/10">
          {t("settings.switchTo")} {theme === "dark" ? t("settings.light") : t("settings.dark")} {t("settings.mode")}
        </button>
      </section>
    </div>
  );
}
