# SavagePC AI 🤖💀

SavagePC AI is a next-generation AI desktop assistant that combines real-time system monitoring, intelligent diagnostics, and a unique sarcastic personality to help users keep their PCs healthy and optimized.

Unlike traditional system monitoring tools, SavagePC AI doesn't just display system statistics—it understands them, explains what's happening in natural language, and can perform maintenance tasks with the user's permission.

Whether your SSD is full, your RAM is overloaded, your CPU is overheating, or you've ignored Windows updates for weeks, SavagePC AI will let you know... probably with a roast.

## Features

- 🤖 AI-powered desktop assistant
- 💬 Natural chat interface
- 😂 Multiple personalities (Friendly, Sarcastic, Savage)
- 🖥️ Real-time system monitoring
- 📊 Live CPU, GPU, RAM, Disk and Network statistics
- 🧹 One-click PC cleanup and optimization
- 🔍 Windows health analysis
- 🔐 Security checks
- ⚡ Startup optimization
- 🎮 Gaming performance analysis
- 🔋 Battery health monitoring
- 🌐 Internet diagnostics
- 🔔 Smart proactive notifications
- 🎤 Voice conversations (planned)
- 🧠 Memory system
- 🔌 Plugin architecture (planned)

## Supported AI Providers

- OpenAI
- Mistral AI
- Google Gemini
- xAI Grok

Users can switch providers at any time using their own API keys.

## Tech Stack

Frontend
- React
- TypeScript
- Tailwind CSS
- Tauri
- Framer Motion

Backend
- Rust
- SQLite
- Windows APIs
- WMI
- Win32
- Function Calling
- Streaming AI

## Philosophy

The assistant never invents system information.

Every recommendation, warning, joke, or roast is based entirely on real system telemetry collected from the user's computer.

Privacy comes first:
- Local-first architecture
- Encrypted API keys
- Minimal data sent to AI providers
- No telemetry collection without user consent

## Current Status

🚧 Active Development

This repo now contains a working scaffold of the full architecture:

- **Frontend** (`src/`): Vite + React + TypeScript + Tailwind, with a chat view, a live system
  dashboard (CPU/RAM/disk/network/process charts via Recharts), a settings view for LLM provider
  configuration, and a friendly/sarcastic/savage personality switcher. Framer Motion powers the UI
  transitions. The frontend runs standalone in a browser with mock telemetry for UI development,
  and talks to the real backend automatically when run inside Tauri.
- **Backend** (`src-tauri/`): Rust + Tauri 2. Includes a real telemetry collector (CPU, RAM, disk,
  network, battery, top processes via `sysinfo` and `starship-battery`), a SQLite-backed memory
  store (`rusqlite`) for settings/history, a unified LLM router supporting OpenAI, Mistral, Gemini,
  and Grok (the first three share an OpenAI-compatible chat-completions dialect; Gemini uses
  Google's `generateContent` API), and a tool-execution engine wired to real OS commands
  (empty recycle bin, delete temp files, clear browser cache, analyze large folders, open task
  manager/settings, kill/launch processes, restart/shutdown/sleep).

Not yet implemented (tracked as follow-up work): SMART disk health, Windows-specific WMI/Event Log
integration, startup app analysis, streaming token-by-token responses, voice I/O, OS-keychain-backed
API key encryption (keys are currently stored locally in SQLite, unencrypted), and the plugin
marketplace / multi-PC features from the "Future Features" section below.

### Running it

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

Build the world's most entertaining and useful desktop AI assistant—one that feels like a real friend living on your computer while helping you maintain, optimize, and understand your system.# SavagePC-AI-The-Funny-Desktop-AI-Agent
A funny AI-powered desktop assistant that monitors your PC in real time, roasts you with personality, and helps optimize Windows using OpenAI, Mistral, Gemini, and Grok.
