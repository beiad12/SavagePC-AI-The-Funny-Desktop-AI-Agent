export default function StatCard({
  label,
  value,
  sub,
  percent,
  danger,
}: {
  label: string;
  value: string;
  sub?: string;
  percent?: number;
  danger?: boolean;
}) {
  return (
    <div className="glass rounded-2xl p-4">
      <div className="text-xs uppercase tracking-wide text-slate-400">{label}</div>
      <div className={`mt-1 text-2xl font-semibold ${danger ? "text-savage-accent" : "text-white"}`}>{value}</div>
      {sub && <div className="mt-0.5 text-xs text-slate-500">{sub}</div>}
      {percent !== undefined && (
        <div className="mt-3 h-1.5 w-full overflow-hidden rounded-full bg-white/10">
          <div
            className={`h-full rounded-full ${danger ? "bg-savage-accent" : "bg-savage-good"}`}
            style={{ width: `${Math.min(100, Math.max(0, percent))}%` }}
          />
        </div>
      )}
    </div>
  );
}
