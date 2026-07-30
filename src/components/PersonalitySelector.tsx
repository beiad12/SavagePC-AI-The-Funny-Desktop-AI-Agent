import type { Personality } from "@/lib/types";
import { useAppStore, useT } from "@/state/store";

const OPTIONS: { id: Personality; key: string; emoji: string }[] = [
  { id: "friendly", key: "personality.friendly", emoji: "🙂" },
  { id: "sarcastic", key: "personality.sarcastic", emoji: "😏" },
  { id: "savage", key: "personality.savage", emoji: "🔥" },
];

export default function PersonalitySelector() {
  const personality = useAppStore((s) => s.personality);
  const setPersonality = useAppStore((s) => s.setPersonality);
  const t = useT();

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
          {opt.emoji} {t(opt.key)}
        </button>
      ))}
    </div>
  );
}
