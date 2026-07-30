import type { LlmProvider } from "./types";

export interface ModelOption {
  id: string;
  label: string;
  tier: "large" | "small";
}

/** Curated presets per provider — large (smartest) and small/flash (fastest, cheapest) variants.
 *  The model field stays a free-text input, so any other model ID can be typed in too. */
export const MODEL_OPTIONS: Record<LlmProvider, ModelOption[]> = {
  openai: [
    { id: "gpt-4o", label: "GPT-4o", tier: "large" },
    { id: "gpt-4.1", label: "GPT-4.1", tier: "large" },
    { id: "o4-mini", label: "o4-mini (reasoning)", tier: "small" },
    { id: "gpt-4o-mini", label: "GPT-4o mini", tier: "small" },
    { id: "gpt-4.1-mini", label: "GPT-4.1 mini", tier: "small" },
    { id: "gpt-4.1-nano", label: "GPT-4.1 nano", tier: "small" },
  ],
  mistral: [
    { id: "mistral-large-latest", label: "Mistral Large", tier: "large" },
    { id: "pixtral-large-latest", label: "Pixtral Large", tier: "large" },
    { id: "mistral-medium-latest", label: "Mistral Medium", tier: "large" },
    { id: "mistral-small-latest", label: "Mistral Small", tier: "small" },
    { id: "ministral-8b-latest", label: "Ministral 8B", tier: "small" },
    { id: "ministral-3b-latest", label: "Ministral 3B", tier: "small" },
  ],
  gemini: [
    { id: "gemini-1.5-pro", label: "Gemini 1.5 Pro", tier: "large" },
    { id: "gemini-2.0-pro-exp", label: "Gemini 2.0 Pro", tier: "large" },
    { id: "gemini-1.5-flash", label: "Gemini 1.5 Flash", tier: "small" },
    { id: "gemini-2.0-flash", label: "Gemini 2.0 Flash", tier: "small" },
    { id: "gemini-2.0-flash-lite", label: "Gemini 2.0 Flash-Lite", tier: "small" },
    { id: "gemini-1.5-flash-8b", label: "Gemini 1.5 Flash-8B", tier: "small" },
  ],
  grok: [
    { id: "grok-2-latest", label: "Grok 2", tier: "large" },
    { id: "grok-2-vision-latest", label: "Grok 2 Vision", tier: "large" },
    { id: "grok-3", label: "Grok 3", tier: "large" },
    { id: "grok-3-mini", label: "Grok 3 mini", tier: "small" },
  ],
};
