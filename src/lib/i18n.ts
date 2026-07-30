export type LanguageCode = "en" | "fr" | "ar" | "ary";

export const LANGUAGES: { code: LanguageCode; label: string; flag: string; rtl: boolean }[] = [
  { code: "en", label: "English", flag: "🇬🇧", rtl: false },
  { code: "fr", label: "Français", flag: "🇫🇷", rtl: false },
  { code: "ar", label: "العربية", flag: "🇸🇦", rtl: true },
  { code: "ary", label: "الدارجة المغربية", flag: "🇲🇦", rtl: true },
];

/** Human-readable name sent to the AI so it knows what to reply in. */
export const LANGUAGE_NAME_FOR_PROMPT: Record<LanguageCode, string> = {
  en: "English",
  fr: "French",
  ar: "Modern Standard Arabic",
  ary: "Moroccan Darija (Moroccan Arabic dialect, written in Arabic script, colloquial)",
};

type Dict = Record<string, string>;

const en: Dict = {
  "nav.chat": "Chat",
  "nav.dashboard": "Dashboard",
  "nav.alerts": "Notifications",
  "nav.settings": "Settings",

  "sidebar.cpu": "CPU",
  "sidebar.ram": "RAM",

  "chat.placeholder": "Ask about your PC, or tell me to clean something…",
  "chat.send": "Send",
  "chat.thinking": "thinking…",
  "chat.error": "Something broke talking to the LLM:",

  "personality.friendly": "Friendly",
  "personality.sarcastic": "Sarcastic",
  "personality.savage": "Savage 18+",

  "dashboard.title": "System Dashboard",
  "dashboard.loading": "Loading telemetry…",
  "dashboard.cpu": "CPU",
  "dashboard.ram": "RAM",
  "dashboard.network": "Network",
  "dashboard.uptime": "Uptime",
  "dashboard.battery": "Battery",
  "dashboard.desktop": "Desktop / no battery",
  "dashboard.usedOf": "used",
  "dashboard.total": "total",
  "dashboard.disk": "Disk",
  "dashboard.free": "free",
  "dashboard.history": "CPU / RAM history",
  "dashboard.topProcesses": "Top processes",
  "dashboard.maintenance": "One-click maintenance",
  "tool.empty_recycle_bin": "Empty Recycle Bin",
  "tool.delete_temp_files": "Delete Temp Files",
  "tool.clear_browser_cache": "Clear Browser Cache",
  "tool.open_task_manager": "Open Task Manager",
  "dashboard.running": "Running…",

  "notifications.title": "Notifications",
  "notifications.subtitle":
    "SavagePC AI keeps watching in the background, even when this window is closed — these are the things it flagged.",
  "notifications.empty": "Nothing to report. Suspiciously quiet. 👀",
  "notifications.dismiss": "Dismiss",

  "settings.title": "Settings",
  "settings.language": "Language",
  "settings.aiProvider": "AI Provider",
  "settings.apiKey": "API Key",
  "settings.model": "Model",
  "settings.save": "Save",
  "settings.saved": "Saved ✓",
  "settings.privacyNote":
    "Keys are encrypted at rest and only sent directly to the provider you choose. No telemetry leaves this device unless it's part of a message you send to the AI.",
  "settings.appearance": "Appearance",
  "settings.switchTo": "Switch to",
  "settings.dark": "dark",
  "settings.light": "light",
  "settings.mode": "mode",
};

const fr: Dict = {
  "nav.chat": "Discussion",
  "nav.dashboard": "Tableau de bord",
  "nav.alerts": "Notifications",
  "nav.settings": "Paramètres",

  "sidebar.cpu": "CPU",
  "sidebar.ram": "RAM",

  "chat.placeholder": "Demande-moi sur ton PC, ou dis-moi de nettoyer quelque chose…",
  "chat.send": "Envoyer",
  "chat.thinking": "réflexion…",
  "chat.error": "Un problème est survenu avec le LLM :",

  "personality.friendly": "Amical",
  "personality.sarcastic": "Sarcastique",
  "personality.savage": "Savage 18+",

  "dashboard.title": "Tableau de bord système",
  "dashboard.loading": "Chargement des données…",
  "dashboard.cpu": "CPU",
  "dashboard.ram": "RAM",
  "dashboard.network": "Réseau",
  "dashboard.uptime": "Disponibilité",
  "dashboard.battery": "Batterie",
  "dashboard.desktop": "Ordinateur de bureau / pas de batterie",
  "dashboard.usedOf": "utilisé",
  "dashboard.total": "total",
  "dashboard.disk": "Disque",
  "dashboard.free": "libre",
  "dashboard.history": "Historique CPU / RAM",
  "dashboard.topProcesses": "Processus principaux",
  "dashboard.maintenance": "Maintenance en un clic",
  "tool.empty_recycle_bin": "Vider la corbeille",
  "tool.delete_temp_files": "Supprimer les fichiers temporaires",
  "tool.clear_browser_cache": "Vider le cache du navigateur",
  "tool.open_task_manager": "Ouvrir le gestionnaire des tâches",
  "dashboard.running": "En cours…",

  "notifications.title": "Notifications",
  "notifications.subtitle":
    "SavagePC AI continue de surveiller en arrière-plan, même quand cette fenêtre est fermée — voici ce qu'il a signalé.",
  "notifications.empty": "Rien à signaler. Suspicieusement calme. 👀",
  "notifications.dismiss": "Ignorer",

  "settings.title": "Paramètres",
  "settings.language": "Langue",
  "settings.aiProvider": "Fournisseur IA",
  "settings.apiKey": "Clé API",
  "settings.model": "Modèle",
  "settings.save": "Enregistrer",
  "settings.saved": "Enregistré ✓",
  "settings.privacyNote":
    "Les clés sont chiffrées localement et envoyées uniquement au fournisseur choisi. Aucune donnée ne quitte cet appareil sauf si elle fait partie d'un message envoyé à l'IA.",
  "settings.appearance": "Apparence",
  "settings.switchTo": "Passer en mode",
  "settings.dark": "sombre",
  "settings.light": "clair",
  "settings.mode": "",
};

const ar: Dict = {
  "nav.chat": "محادثة",
  "nav.dashboard": "لوحة التحكم",
  "nav.alerts": "الإشعارات",
  "nav.settings": "الإعدادات",

  "sidebar.cpu": "المعالج",
  "sidebar.ram": "الذاكرة",

  "chat.placeholder": "اسألني عن جهازك، أو قل لي أن أنظّف شيئًا…",
  "chat.send": "إرسال",
  "chat.thinking": "أفكّر…",
  "chat.error": "حدث خطأ أثناء التواصل مع النموذج:",

  "personality.friendly": "ودود",
  "personality.sarcastic": "ساخر",
  "personality.savage": "وحشي 18+",

  "dashboard.title": "لوحة تحكم النظام",
  "dashboard.loading": "جارٍ تحميل البيانات…",
  "dashboard.cpu": "المعالج",
  "dashboard.ram": "الذاكرة العشوائية",
  "dashboard.network": "الشبكة",
  "dashboard.uptime": "مدة التشغيل",
  "dashboard.battery": "البطارية",
  "dashboard.desktop": "حاسوب مكتبي / بدون بطارية",
  "dashboard.usedOf": "مستخدم",
  "dashboard.total": "الإجمالي",
  "dashboard.disk": "القرص",
  "dashboard.free": "متاح",
  "dashboard.history": "سجلّ المعالج والذاكرة",
  "dashboard.topProcesses": "أكثر العمليات استهلاكًا",
  "dashboard.maintenance": "صيانة بنقرة واحدة",
  "tool.empty_recycle_bin": "إفراغ سلة المحذوفات",
  "tool.delete_temp_files": "حذف الملفات المؤقتة",
  "tool.clear_browser_cache": "مسح ذاكرة التخزين المؤقت للمتصفح",
  "tool.open_task_manager": "فتح مدير المهام",
  "dashboard.running": "جارٍ التنفيذ…",

  "notifications.title": "الإشعارات",
  "notifications.subtitle": "يستمر SavagePC AI في المراقبة في الخلفية حتى عند إغلاق هذه النافذة — هذه هي الأمور التي رصدها.",
  "notifications.empty": "لا يوجد شيء يُذكر. هدوء مثير للريبة. 👀",
  "notifications.dismiss": "إخفاء",

  "settings.title": "الإعدادات",
  "settings.language": "اللغة",
  "settings.aiProvider": "مزوّد الذكاء الاصطناعي",
  "settings.apiKey": "مفتاح API",
  "settings.model": "النموذج",
  "settings.save": "حفظ",
  "settings.saved": "تم الحفظ ✓",
  "settings.privacyNote":
    "يتم تشفير المفاتيح محليًا وإرسالها فقط إلى المزوّد الذي تختاره. لا تغادر أي بيانات هذا الجهاز إلا إذا كانت جزءًا من رسالة أرسلتها إلى الذكاء الاصطناعي.",
  "settings.appearance": "المظهر",
  "settings.switchTo": "التبديل إلى الوضع",
  "settings.dark": "الداكن",
  "settings.light": "الفاتح",
  "settings.mode": "",
};

const ary: Dict = {
  "nav.chat": "الشات",
  "nav.dashboard": "لوحة التحكم",
  "nav.alerts": "التنبيهات",
  "nav.settings": "الإعدادات",

  "sidebar.cpu": "المعالج",
  "sidebar.ram": "الرام",

  "chat.placeholder": "سولني على البيسي ديالك، ولا قوليا نصاوب شي حاجة…",
  "chat.send": "صيفط",
  "chat.thinking": "كيخمم…",
  "chat.error": "وقع مشكل مع النموذج:",

  "personality.friendly": "لطيف",
  "personality.sarcastic": "ساخر",
  "personality.savage": "وحشي 18+",

  "dashboard.title": "لوحة تحكم الجهاز",
  "dashboard.loading": "كيتحمّل الجهاز…",
  "dashboard.cpu": "المعالج",
  "dashboard.ram": "الرام",
  "dashboard.network": "الأنترنت",
  "dashboard.uptime": "مدة التشغيل",
  "dashboard.battery": "الباطري",
  "dashboard.desktop": "ديسكتوب / ماكاينش باطري",
  "dashboard.usedOf": "مستعمل",
  "dashboard.total": "المجموع",
  "dashboard.disk": "الديسك",
  "dashboard.free": "خاوي",
  "dashboard.history": "تاريخ المعالج و الرام",
  "dashboard.topProcesses": "البرامج اللي كتاكل بزاف",
  "dashboard.maintenance": "التنظيف بضغطة وحدة",
  "tool.empty_recycle_bin": "فرّغ المهملات",
  "tool.delete_temp_files": "حذف الملفات المؤقتة",
  "tool.clear_browser_cache": "مسح الكاش ديال المتصفح",
  "tool.open_task_manager": "حل مدير المهام",
  "dashboard.running": "كيخدم…",

  "notifications.title": "التنبيهات",
  "notifications.subtitle": "SavagePC AI بقى يراقب فالخلفية، حتى إلا سديتي النافذة — هادي هي الحوايج اللي شاف فيهم مشكل.",
  "notifications.empty": "ماكاين والو. سكات مشبوه شويا. 👀",
  "notifications.dismiss": "خبّي",

  "settings.title": "الإعدادات",
  "settings.language": "اللغة",
  "settings.aiProvider": "مزوّد الذكاء الاصطناعي",
  "settings.apiKey": "مفتاح API",
  "settings.model": "الموديل",
  "settings.save": "سجل",
  "settings.saved": "تسجل ✓",
  "settings.privacyNote":
    "المفاتيح كيتشفراو ف الجهاز ديالك وكيتصيفطو غير للمزوّد اللي خترتي. مايخرج والو من الجهاز إلا إلا كان جزء من رسالة صيفطيتيها للذكاء الاصطناعي.",
  "settings.appearance": "المظهر",
  "settings.switchTo": "بدّل لوضع",
  "settings.dark": "المظلم",
  "settings.light": "الفاتح",
  "settings.mode": "",
};

export const TRANSLATIONS: Record<LanguageCode, Dict> = { en, fr, ar, ary };

export function translate(lang: LanguageCode, key: string): string {
  return TRANSLATIONS[lang]?.[key] ?? TRANSLATIONS.en[key] ?? key;
}
