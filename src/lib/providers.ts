import type { LlmProvider } from "./types";

export const PROVIDERS: { id: LlmProvider; label: string }[] = [
  { id: "openai", label: "OpenAI" },
  { id: "mistral", label: "Mistral AI" },
  { id: "gemini", label: "Google Gemini" },
  { id: "grok", label: "xAI Grok" },
];
