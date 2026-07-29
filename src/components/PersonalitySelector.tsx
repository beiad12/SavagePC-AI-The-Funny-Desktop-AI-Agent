import type { Personality } from "@/lib/types";
import { useAppStore } from "@/state/store";

const OPTIONS: { id: Personality; label: string; emoji: string }[] = [
  { id: "friendly", label: "Friendly", emoji: "🙂" },
  { id: "sarcastic", label: "Sarcastic", emoji: "😏" },
  { id: "savage", label: "Savage 18+", emoji: "🔥" },
];

export default function PersonalitySelector() {
  const personality = useAppStore((s) => s.personality);
  const setPersonality = useAppStore((s) => s.setPersonality);

  return (
    <div className="flex gap-1 rounded-full bg-white/5 p-1">
      {OPTIONS.map((opt) => (
        <button
          key={opt.id}
          onClick={() => setPersonality(opt.id)}
          className={`rounded-full px-3 py-1 text-xs font-medium transition ${
            personality === opt.id ? "bg-white/15 text-white" : "text-slate-400 hover:text-slate-200"
          }`}
        >
          {opt.emoji} {opt.label}
        </button>
      ))}
    </div>
  );
}
