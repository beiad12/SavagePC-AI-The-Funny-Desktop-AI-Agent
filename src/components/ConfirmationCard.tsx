import { useState } from "react";
import { runTool } from "@/lib/bridge";
import { useAppStore, useT } from "@/state/store";
import type { ChatMessage, PendingConfirmation } from "@/lib/types";

export default function ConfirmationCard({ pending }: { pending: PendingConfirmation }) {
  const [status, setStatus] = useState<"pending" | "running" | "resolved">("pending");
  const addMessage = useAppStore((s) => s.addMessage);
  const t = useT();

  const confirm = async () => {
    setStatus("running");
    try {
      const result = await runTool(pending.name, pending.args);
      const msg: ChatMessage = {
        id: crypto.randomUUID(),
        role: "assistant",
        content: `✅ ${result}`,
        createdAt: Date.now(),
      };
      addMessage(msg);
    } catch (err) {
      addMessage({
        id: crypto.randomUUID(),
        role: "assistant",
        content: `${t("chat.error")} ${String(err)}`,
        createdAt: Date.now(),
      });
    } finally {
      setStatus("resolved");
    }
  };

  const cancel = () => {
    setStatus("resolved");
    addMessage({
      id: crypto.randomUUID(),
      role: "assistant",
      content: t("chat.cancelled"),
      createdAt: Date.now(),
    });
  };

  if (status === "resolved") return null;

  return (
    <div className="mt-1 rounded-xl border border-savage-accent/40 bg-savage-accent/10 p-3">
      <div className="mb-1 flex items-center gap-1.5 text-xs">
        <span>⚠️</span>
        <span className="font-mono font-semibold text-savage-accent">{pending.name}</span>
        {Object.keys(pending.args).length > 0 && (
          <span className="font-mono text-slate-400">({Object.values(pending.args).join(", ")})</span>
        )}
      </div>
      <p className="mb-2 text-xs text-slate-400">{pending.description}</p>
      <div className="flex gap-2">
        <button
          onClick={confirm}
          disabled={status === "running"}
          className="rounded-lg bg-savage-accent px-3 py-1.5 text-xs font-medium text-white disabled:opacity-50"
        >
          {status === "running" ? "…" : t("chat.confirm")}
        </button>
        <button
          onClick={cancel}
          disabled={status === "running"}
          className="rounded-lg bg-white/10 px-3 py-1.5 text-xs hover:bg-white/15 disabled:opacity-50"
        >
          {t("chat.cancel")}
        </button>
      </div>
    </div>
  );
}
