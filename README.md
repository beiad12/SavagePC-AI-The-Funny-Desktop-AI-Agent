# SavagePC AI 🤖💀

SavagePC AI is a desktop AI agent that doesn't just monitor your computer — it talks about it in the first person, like the PC itself has feelings, and it can actually *do* things about what it finds.

Unlike traditional system monitoring tools, SavagePC AI understands your telemetry, explains what's happening in natural language (in your chosen personality and language), and takes real action — closing apps, cleaning up disk space, scanning for what's eating your storage — through real function calling, not just talk.

Whether your SSD is full, your RAM is overloaded, your CPU is overheating, or your battery's about to die, SavagePC AI will let you know... probably with a roast, in whatever language and tone you prefer.

## 📥 Download

Grab the latest Windows installer: **[`installer/SavagePC-AI-Setup.exe`](installer/SavagePC-AI-Setup.exe)**

Run it like any normal Windows installer — no extra dependencies. You'll need an API key from one of the supported providers (see below); the first-run setup wizard will ask for it.

> That path is kept up to date in place — only the latest build lives there, old ones aren't left behind.

## ✨ Features

### 🗣️ A personality grounded in real telemetry, not invented
- Speaks in first person as the PC itself: 🧠 RAM is its lungs (suffocates when full), ❤️ CPU is its heart (races when hot), 🍔 disk is its stomach, 🔋 battery is its hunger — every line backed by real numbers, never made up.
- Three tones: 🙂 Friendly · 😏 Sarcastic · 🔥 Savage (18+)
- Four languages: English, Français, العربية (Modern Standard Arabic), and **الدارجة المغربية** (real Moroccan Darija, not machine-translated MSA) — the whole UI, chat, and notifications switch together, with full RTL layout for Arabic/Darija.

### 🛠️ A real agent, not just a chatbot
- True function-calling across all 4 providers — asking it to close an app, empty the recycle bin, or open a specific folder path actually executes the action and reports back the real result.
- Real confirmation dialogs: destructive actions (killing a process, restarting, clearing caches, etc.) show a genuine Confirm/Cancel card in the chat before anything runs — not just the AI asking nicely in text.
- Live disk scanning with a real progress animation: "why is my PC slow?" or "find my large files" triggers an actual filesystem scan (with a live scanning bubble showing item count + current path), then a synthesized diagnosis with concrete recommendations.
- Open/close arbitrary paths by name — "open C:\Users\MEHDI" opens it in Explorer, "close it" actually closes that window.

### 🌙 Runs like a real background service
- Closing the window minimizes to the system tray instead of quitting — a background monitor keeps polling CPU/RAM/disk/battery even while the window is closed.
- Notifications are generated fresh by the AI each time (not repeated canned text), in your personality/language, including a happy reaction when you plug in to charge instead of just more complaints.

### 📊 Live system dashboard
- Real-time CPU, RAM, disk, network, and battery stats with history charts and a top-processes breakdown.
- One-click maintenance: empty recycle bin, delete temp files, clear browser cache, open Task Manager.

### 🚀 Setup that doesn't dump you into an empty chat
- First-run onboarding wizard: pick your AI provider → add API key/model → pick personality & language.
- Curated model presets per provider split into Large (smartest) vs. Small/Flash (fastest, cheapest) tiers, plus a free-text field for any custom model ID.

## Supported AI Providers

- OpenAI
- Mistral AI
- Google Gemini
- xAI Grok

Switch providers and models at any time in Settings, using your own API keys.

## Tech Stack

Frontend
- React + TypeScript
- Tailwind CSS
- Tauri 2
- Framer Motion
- react-markdown

Backend
- Rust
- SQLite (`rusqlite`)
- `sysinfo` / `starship-battery` for telemetry
- Tauri's notification, tray-icon, and event systems
- Real function/tool calling across OpenAI, Mistral, Gemini, and Grok

## Philosophy

The assistant never invents system information. Every joke, warning, or recommendation is grounded in real telemetry collected from the user's own computer, and every action it claims to have taken was actually executed and confirmed via a real tool call — not just written in a chat bubble.

Privacy:
- Local-first architecture
- Minimal data sent to AI providers (only what's needed for the current message)
- No telemetry collection without user consent

⚠️ **Known limitation:** API keys are currently stored locally in SQLite, unencrypted (no OS-keychain integration yet).

## Current Status

🚧 Active Development — v0.1.0 is the first installable build (see Download above).

**Not yet implemented:** SMART disk health, startup-app analysis, streaming token-by-token responses, voice I/O, OS-keychain-backed API key encryption, gaming-library detection, and the plugin marketplace / multi-PC features.

### Running it from source

```bash
npm install
npm run tauri dev   # full desktop app (requires Rust toolchain + platform WebView deps)
# or, for frontend-only UI iteration with mock data:
npm run dev
```

### Building a Windows installer

The project builds natively on Windows with:

```bash
npm install
npm run tauri build   # produces an NSIS setup.exe under src-tauri/target/release/bundle/nsis/
```

It can also be cross-compiled from Linux for smoke-testing (experimental — WebView2 and Windows
APIs are only truly exercised on real Windows):

```bash
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64 nsis
npm run tauri build -- --target x86_64-pc-windows-gnu --bundles nsis
```

`src-tauri/.cargo/config.toml` points that target at the mingw-w64 linker. Note the app's Rust
library crate only builds as `staticlib`/`rlib` (no `cdylib`) — GNU `ld`'s PE export-ordinal limit
is otherwise exceeded when cross-linking Tauri's dependency tree into a DLL.

## Vision

Build the world's most entertaining and useful desktop AI assistant — one that feels like a real friend living on your computer, that actually acts on your behalf, while helping you maintain, optimize, and understand your system.
