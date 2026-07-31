import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { saveLanguage, savePersonality, saveProviderConfig } from "@/lib/bridge";
import { MODEL_OPTIONS } from "@/lib/models";
import { PROVIDERS } from "@/lib/providers";
import { LANGUAGES } from "@/lib/i18n";
import { useAppStore, useT, DEFAULT_MODELS_MAP } from "@/state/store";
import type { LlmProvider, Personality } from "@/lib/types";

const PERSONALITIES: { id: Personality; key: string; emoji: string }[] = [
  { id: "friendly", key: "personality.friendly", emoji: "🙂" },
  { id: "sarcastic", key: "personality.sarcastic", emoji: "😏" },
  { id: "savage", key: "personality.savage", emoji: "🔥" },
];

export default function OnboardingWizard({ onComplete }: { onComplete: () => void }) {
  const [step, setStep] = useState(0);
  const [providerId, setProviderId] = useState<LlmProvider>("openai");
  const [apiKey, setApiKey] = useState("");
  const [model, setModel] = useState(DEFAULT_MODELS_MAP.openai);
  const [personality, setPersonalityLocal] = useState<Personality>("sarcastic");
  const [language, setLanguageLocal] = useState<(typeof LANGUAGES)[number]["code"]>("en");
  const setProvider = useAppStore((s) => s.setProvider);
  const setPersonalityGlobal = useAppStore((s) => s.setPersonality);
  const setLanguageGlobal = useAppStore((s) => s.setLanguage);
  const t = useT();

  const chooseProvider = (id: LlmProvider) => {
    setProviderId(id);
    setModel(DEFAULT_MODELS_MAP[id]);
  };

  const finish = async () => {
    const config = { provider: providerId, apiKey, model };
    setProvider(config);
    setPersonalityGlobal(personality);
    setLanguageGlobal(language);
    await Promise.all([saveProviderConfig(config), savePersonality(personality), saveLanguage(language)]);
    onComplete();
  };

  const canProceed = step === 1 ? apiKey.trim().length > 0 : true;

  return (
    <div className="flex h-full w-full items-center justify-center bg-savage-bg p-6">
      <div className="glass w-full max-w-lg rounded-3xl p-8">
        <div className="mb-6 flex items-center justify-between">
          <div>
            <h1 className="text-lg font-semibold">{t("onboarding.title")}</h1>
            <p className="text-sm text-slate-500">{t("onboarding.subtitle")}</p>
          </div>
          <button onClick={onComplete} className="text-xs text-slate-500 hover:text-slate-300">
            {t("onboarding.skip")}
          </button>
        </div>

        <div className="mb-6 flex gap-1.5">
          {[0, 1, 2].map((i) => (
            <div
              key={i}
              className={`h-1.5 flex-1 rounded-full transition ${i <= step ? "bg-savage-accent" : "bg-white/10"}`}
            />
          ))}
        </div>

        <AnimatePresence mode="wait">
          {step === 0 && (
            <motion.div
              key="step0"
              initial={{ opacity: 0, x: 12 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -12 }}
            >
              <h2 className="mb-3 text-sm font-semibold text-slate-300">{t("onboarding.step1")}</h2>
              <div className="grid grid-cols-2 gap-2">
                {PROVIDERS.map((p) => (
                  <button
                    key={p.id}
                    onClick={() => chooseProvider(p.id)}
                    className={`rounded-xl px-3 py-3 text-sm transition ${
                      providerId === p.id ? "bg-white/15 text-white" : "bg-white/5 text-slate-400 hover:bg-white/10"
                    }`}
                  >
                    {p.label}
                  </button>
                ))}
              </div>
            </motion.div>
          )}

          {step === 1 && (
            <motion.div
              key="step1"
              initial={{ opacity: 0, x: 12 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -12 }}
            >
              <h2 className="mb-3 text-sm font-semibold text-slate-300">{t("onboarding.step2")}</h2>
              <label className="mb-1 block text-xs text-slate-400">{t("settings.apiKey")}</label>
              <input
                type="password"
                value={apiKey}
                onChange={(e) => setApiKey(e.target.value)}
                placeholder="sk-…"
                autoFocus
                className="mb-4 w-full rounded-xl bg-white/5 px-4 py-2 text-sm outline-none focus:bg-white/10"
              />
              <div className="mb-1 text-[10px] uppercase tracking-wide text-slate-500">{t("settings.modelLarge")}</div>
              <div className="mb-2 flex flex-wrap gap-1.5">
                {MODEL_OPTIONS[providerId]
                  .filter((m) => m.tier === "large")
                  .map((m) => (
                    <button
                      key={m.id}
                      onClick={() => setModel(m.id)}
                      className={`rounded-lg px-2.5 py-1 text-xs transition ${
                        model === m.id ? "bg-white/15 text-white" : "bg-white/5 text-slate-400 hover:bg-white/10"
                      }`}
                    >
                      {m.label}
                    </button>
                  ))}
              </div>
              <div className="mb-1 text-[10px] uppercase tracking-wide text-slate-500">{t("settings.modelSmall")}</div>
              <div className="flex flex-wrap gap-1.5">
                {MODEL_OPTIONS[providerId]
                  .filter((m) => m.tier === "small")
                  .map((m) => (
                    <button
                      key={m.id}
                      onClick={() => setModel(m.id)}
                      className={`rounded-lg px-2.5 py-1 text-xs transition ${
                        model === m.id ? "bg-white/15 text-white" : "bg-white/5 text-slate-400 hover:bg-white/10"
                      }`}
                    >
                      {m.label}
                    </button>
                  ))}
              </div>
            </motion.div>
          )}

          {step === 2 && (
            <motion.div
              key="step2"
              initial={{ opacity: 0, x: 12 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -12 }}
            >
              <h2 className="mb-3 text-sm font-semibold text-slate-300">{t("onboarding.step3")}</h2>
              <div className="mb-4 flex gap-1.5 rounded-full bg-white/5 p-1">
                {PERSONALITIES.map((p) => (
                  <button
                    key={p.id}
                    onClick={() => setPersonalityLocal(p.id)}
                    className={`flex-1 rounded-full px-3 py-1.5 text-xs font-medium transition ${
                      personality === p.id ? "bg-white/15 text-white" : "text-slate-400 hover:text-slate-200"
                    }`}
                  >
                    {p.emoji} {t(p.key)}
                  </button>
                ))}
              </div>
              <div className="mb-1 text-xs text-slate-400">{t("settings.language")}</div>
              <div className="grid grid-cols-2 gap-2">
                {LANGUAGES.map((lang) => (
                  <button
                    key={lang.code}
                    onClick={() => setLanguageLocal(lang.code)}
                    className={`rounded-xl px-3 py-2 text-sm transition ${
                      language === lang.code ? "bg-white/15 text-white" : "bg-white/5 text-slate-400 hover:bg-white/10"
                    }`}
                  >
                    {lang.flag} {lang.label}
                  </button>
                ))}
              </div>
            </motion.div>
          )}
        </AnimatePresence>

        <div className="mt-6 flex justify-between">
          <button
            onClick={() => setStep((s) => Math.max(0, s - 1))}
            disabled={step === 0}
            className="rounded-xl bg-white/5 px-4 py-2 text-sm hover:bg-white/10 disabled:opacity-0"
          >
            {t("onboarding.back")}
          </button>
          {step < 2 ? (
            <button
              onClick={() => setStep((s) => Math.min(2, s + 1))}
              disabled={!canProceed}
              className="rounded-xl bg-gradient-to-br from-savage-accent2 to-savage-accent px-5 py-2 text-sm font-medium disabled:opacity-40"
            >
              {t("onboarding.next")}
            </button>
          ) : (
            <button
              onClick={finish}
              className="rounded-xl bg-gradient-to-br from-savage-accent2 to-savage-accent px-5 py-2 text-sm font-medium"
            >
              {t("onboarding.finish")}
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
