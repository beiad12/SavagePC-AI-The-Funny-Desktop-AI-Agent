import { create } from "zustand";
import type { ChatMessage, LlmProvider, Personality, ProviderConfig, Telemetry } from "@/lib/types";

interface AppState {
  personality: Personality;
  setPersonality: (p: Personality) => void;

  theme: "dark" | "light";
  toggleTheme: () => void;

  provider: ProviderConfig;
  setProvider: (p: Partial<ProviderConfig>) => void;

  telemetry: Telemetry | null;
  setTelemetry: (t: Telemetry) => void;

  messages: ChatMessage[];
  addMessage: (m: ChatMessage) => void;
  setMessages: (m: ChatMessage[]) => void;
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
        "I'm SavagePC AI. I watch your machine so you don't have to pretend you know what's eating your RAM. Ask me anything, or hit a maintenance button and let's see what disaster we're dealing with.",
      createdAt: Date.now(),
    },
  ],
  addMessage: (m) => set((s) => ({ messages: [...s.messages, m] })),
  setMessages: (m) => set({ messages: m }),
}));

export const DEFAULT_MODELS_MAP = DEFAULT_MODELS;
