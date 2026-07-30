import { create } from "zustand";
import type { Alert, ChatMessage, LlmProvider, Personality, ProviderConfig, Telemetry } from "@/lib/types";
import { type LanguageCode, translate } from "@/lib/i18n";

interface AppState {
  personality: Personality;
  setPersonality: (p: Personality) => void;

  language: LanguageCode;
  setLanguage: (l: LanguageCode) => void;

  theme: "dark" | "light";
  toggleTheme: () => void;

  provider: ProviderConfig;
  setProvider: (p: Partial<ProviderConfig>) => void;

  telemetry: Telemetry | null;
  setTelemetry: (t: Telemetry) => void;

  messages: ChatMessage[];
  addMessage: (m: ChatMessage) => void;
  setMessages: (m: ChatMessage[]) => void;

  alerts: Alert[];
  setAlerts: (a: Alert[]) => void;
}

const DEFAULT_MODELS: Record<LlmProvider, string> = {
  openai: "gpt-4o-mini",
  mistral: "mistral-large-latest",
  gemini: "gemini-1.5-pro",
  grok: "grok-2-latest",
};

export const useAppStore = create<AppState>((set, get) => ({
  personality: "sarcastic",
  setPersonality: (p) => set({ personality: p }),

  language: "en",
  setLanguage: (l) => set({ language: l }),

  theme: "dark",
  toggleTheme: () => set({ theme: get().theme === "dark" ? "light" : "dark" }),

  provider: { provider: "openai", apiKey: "", model: DEFAULT_MODELS.openai },
  setProvider: (p) =>
    set((s) => ({
      provider: {
        ...s.provider,
        ...p,
        model: p.provider && p.provider !== s.provider.provider ? DEFAULT_MODELS[p.provider] : p.model ?? s.provider.model,
      },
    })),

  telemetry: null,
  setTelemetry: (t) => set({ telemetry: t }),

  messages: [
    {
      id: "welcome",
      role: "assistant",
      content:
        "Hey, it's me — your PC. I'm not just watching your specs from the outside, I *am* the specs, so when I'm struggling you'll hear about it firsthand 😅 Ask me how I'm feeling, or hit a maintenance button and help me breathe.",
      createdAt: Date.now(),
    },
  ],
  addMessage: (m) => set((s) => ({ messages: [...s.messages, m] })),
  setMessages: (m) => set({ messages: m }),

  alerts: [],
  setAlerts: (a) => set({ alerts: a }),
}));

export const DEFAULT_MODELS_MAP = DEFAULT_MODELS;

/** Subscribes to the current language so components re-render on change. */
export function useT() {
  const language = useAppStore((s) => s.language);
  return (key: string) => translate(language, key);
}
