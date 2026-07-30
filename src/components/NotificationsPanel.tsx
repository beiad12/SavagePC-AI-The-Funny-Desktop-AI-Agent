import { acknowledgeAlert } from "@/lib/bridge";
import { useAppStore, useT } from "@/state/store";

function timeAgo(unixSeconds: number) {
  const diff = Date.now() / 1000 - unixSeconds;
  if (diff < 60) return "just now";
  if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
  if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
  return `${Math.floor(diff / 86400)}d ago`;
}

export default function NotificationsPanel() {
  const alerts = useAppStore((s) => s.alerts);
  const setAlerts = useAppStore((s) => s.setAlerts);
  const t = useT();

  const dismiss = async (id: number) => {
    setAlerts(alerts.map((a) => (a.id === id ? { ...a, acknowledged: true } : a)));
    await acknowledgeAlert(id);
  };

  return (
    <div className="h-full overflow-y-auto p-6">
      <h1 className="mb-1 text-xl font-semibold">{t("notifications.title")}</h1>
      <p className="mb-4 text-sm text-slate-500">{t("notifications.subtitle")}</p>

      {alerts.length === 0 && (
        <div className="glass rounded-2xl p-6 text-center text-sm text-slate-400">{t("notifications.empty")}</div>
      )}

      <div className="flex flex-col gap-2">
        {alerts.map((a) => (
          <div
            key={a.id}
            className={`glass flex items-start justify-between gap-3 rounded-2xl p-4 ${
              a.acknowledged ? "opacity-50" : ""
            }`}
          >
            <div>
              <div
                className={`mb-1 inline-block rounded-full px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide ${
                  a.severity === "critical" ? "bg-savage-accent/20 text-savage-accent" : "bg-savage-warn/20 text-savage-warn"
                }`}
              >
                {a.severity}
              </div>
              <p className="text-sm text-slate-200">{a.message}</p>
              <p className="mt-1 text-xs text-slate-500">{timeAgo(a.created_at)}</p>
            </div>
            {!a.acknowledged && (
              <button
                onClick={() => dismiss(a.id)}
                className="shrink-0 rounded-lg bg-white/5 px-3 py-1.5 text-xs hover:bg-white/10"
              >
                {t("notifications.dismiss")}
              </button>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
