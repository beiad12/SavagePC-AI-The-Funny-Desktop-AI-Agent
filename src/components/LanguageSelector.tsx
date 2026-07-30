import { saveLanguage } from "@/lib/bridge";
import { LANGUAGES } from "@/lib/i18n";
import { useAppStore } from "@/state/store";

export default function LanguageSelector() {
  const language = useAppStore((s) => s.language);
  const setLanguage = useAppStore((s) => s.setLanguage);

  const choose = async (code: (typeof LANGUAGES)[number]["code"]) => {
    setLanguage(code);
    await saveLanguage(code);
  };

  return (
    <div className="grid grid-cols-2 gap-2 lg:grid-cols-4">
      {LANGUAGES.map((lang) => (
        <button
          key={lang.code}
          onClick={() => choose(lang.code)}
          className={`rounded-xl px-3 py-2 text-sm transition ${
            language === lang.code ? "bg-white/15 text-white" : "bg-white/5 text-slate-400 hover:bg-white/10"
          }`}
        >
          {lang.flag} {lang.label}
        </button>
      ))}
    </div>
  );
}
