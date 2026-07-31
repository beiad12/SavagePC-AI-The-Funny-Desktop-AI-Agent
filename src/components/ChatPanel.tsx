import { useEffect, useRef, useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import PersonalitySelector from "@/components/PersonalitySelector";
import MessageContent from "@/components/MessageContent";
import { onScanEvents, sendChatMessage, type ScanProgress } from "@/lib/bridge";
import { useAppStore, useT } from "@/state/store";
import type { ChatMessage } from "@/lib/types";

export default function ChatPanel() {
  const messages = useAppStore((s) => s.messages);
  const addMessage = useAppStore((s) => s.addMessage);
  const personality = useAppStore((s) => s.personality);
  const provider = useAppStore((s) => s.provider);
  const language = useAppStore((s) => s.language);
  const [input, setInput] = useState("");
  const [sending, setSending] = useState(false);
  const [scan, setScan] = useState<ScanProgress | null>(null);
  const bottomRef = useRef<HTMLDivElement>(null);
  const t = useT();

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages, scan]);

  useEffect(() => {
    let unsubscribe: (() => void) | undefined;
    onScanEvents(
      (p) => setScan(p),
      () => setScan(null),
    ).then((fn) => {
      unsubscribe = fn;
    });
    return () => unsubscribe?.();
  }, []);

  const send = async () => {
    const text = input.trim();
    if (!text || sending) return;
    const userMsg: ChatMessage = {
      id: crypto.randomUUID(),
      role: "user",
      content: text,
      createdAt: Date.now(),
    };
    addMessage(userMsg);
    setInput("");
    setSending(true);
    setScan(null);
    try {
      const reply = await sendChatMessage([...messages, userMsg], personality, provider, language);
      addMessage(reply);
    } catch (err) {
      addMessage({
        id: crypto.randomUUID(),
        role: "assistant",
        content: `${t("chat.error")} ${String(err)}`,
        createdAt: Date.now(),
      });
    } finally {
      setSending(false);
      setScan(null);
    }
  };

  return (
    <div className="flex h-full flex-col">
      <header className="glass flex items-center justify-between border-b border-white/5 px-6 py-3">
        <div>
          <h1 className="text-sm font-semibold">SavagePC AI</h1>
          <p className="text-xs text-slate-500">
            {provider.provider.toUpperCase()} · {provider.model}
          </p>
        </div>
        <PersonalitySelector />
      </header>

      <div className="flex-1 overflow-y-auto px-6 py-4">
        <div className="mx-auto flex max-w-2xl flex-col gap-3">
          <AnimatePresence initial={false}>
            {messages.map((m) => (
              <motion.div
                key={m.id}
                initial={{ opacity: 0, y: 8 }}
                animate={{ opacity: 1, y: 0 }}
                className={`max-w-[85%] rounded-2xl px-4 py-2 text-sm leading-relaxed ${
                  m.role === "user"
                    ? "ml-auto bg-gradient-to-br from-savage-accent2 to-savage-accent text-white"
                    : "glass text-slate-200"
                }`}
              >
                {m.role === "user" ? m.content : <MessageContent content={m.content} />}
                {m.toolCalls && m.toolCalls.length > 0 && (
                  <div className="mt-2 flex flex-col gap-1 border-t border-white/10 pt-2">
                    {m.toolCalls.map((tc, i) => (
                      <div key={i} className="flex items-start gap-1.5 text-[11px] text-slate-400">
                        <span>🔧</span>
                        <span>
                          <span className="font-mono text-slate-300">{tc.name}</span>
                          {Object.keys(tc.args).length > 0 && (
                            <span className="font-mono text-slate-500">({Object.values(tc.args).join(", ")})</span>
                          )}
                          {tc.result && <span className="text-slate-500"> — {tc.result}</span>}
                        </span>
                      </div>
                    ))}
                  </div>
                )}
              </motion.div>
            ))}
          </AnimatePresence>

          {sending && scan && (
            <motion.div
              initial={{ opacity: 0, y: 8 }}
              animate={{ opacity: 1, y: 0 }}
              className="glass max-w-[85%] rounded-2xl px-4 py-3 text-sm text-slate-300"
            >
              <div className="flex items-center gap-2">
                <span className="inline-block h-2 w-2 animate-ping rounded-full bg-savage-accent2" />
                <span className="font-medium">{t("chat.scanning")}</span>
              </div>
              <div className="mt-1 truncate font-mono text-[11px] text-slate-500">
                {scan.scanned.toLocaleString()} · {scan.current_path}
              </div>
            </motion.div>
          )}
          {sending && !scan && (
            <div className="glass max-w-[60%] rounded-2xl px-4 py-2 text-sm text-slate-400">{t("chat.thinking")}</div>
          )}
          <div ref={bottomRef} />
        </div>
      </div>

      <div className="border-t border-white/5 p-4">
        <div className="mx-auto flex max-w-2xl items-center gap-2">
          <input
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && send()}
            placeholder={t("chat.placeholder")}
            className="flex-1 rounded-xl bg-white/5 px-4 py-3 text-sm outline-none placeholder:text-slate-500 focus:bg-white/10"
          />
          <button
            onClick={send}
            disabled={sending}
            className="rounded-xl bg-gradient-to-br from-savage-accent2 to-savage-accent px-4 py-3 text-sm font-medium disabled:opacity-50"
          >
            {t("chat.send")}
          </button>
        </div>
      </div>
    </div>
  );
}
