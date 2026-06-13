# 👻 Peekaboo

> Interactive scare overlay for streamers — let your viewers jumpscare you in real time via bits & subs.

![License](https://img.shields.io/badge/license-MIT-blue)
![Tauri](https://img.shields.io/badge/built%20with-Tauri%20v1-orange)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS-lightgrey)

---

## What is this?

Peekaboo is a lightweight desktop app that puts a **transparent, click-through overlay** on top of everything on your screen — including your game. Viewers can trigger jumpscares, screen shakes, glitches, and more via **Twitch or Youtube or Kick**, and the streamer actually sees it happen in real time.

Unlike browser-source-only overlays, **this appears on your monitor** — so the reaction is genuine.

---

## Features

- 👻 **Jumpscare** — full-screen image/GIF + sound effect  
- 📳 **Screen shake** — camera shake feel via CSS animation  
- 📺 **Glitch effect** — RGB split + scanline corruption  
- 💬 **Big text** — huge text that slams onto screen  
- 🎮 **Multi-platform** — Twitch (working) + YouTube/Kick (stubs)  
- ⌨️ **Emergency hotkeys** — kill overlay instantly  
- 🪟 **Click-through** — streamer can still use their PC normally  
- 💾 **Auto-save** — all settings persist via localStorage  
- 🔓 **Open source** — full code, nothing hidden  

---

## Requirements

- [Rust](https://rustup.rs/) (stable ≥ 1.70)
- [Node.js](https://nodejs.org/) (≥ 18)

Dependencies install automatically via `npm install`.

---

## Getting started

```bash
# 1. Clone
git clone https://github.com/you/peekaboo
cd peekaboo

# 2. Install dependencies
npm install

# 3. Run in dev mode
npm run tauri:dev

# 4. Build a release binary
npm run tauri:build
# → installer appears in src-tauri/target/release/bundle/
```

See **[QUICKSTART.md](guide/QUICKSTART.md)** for detailed setup.

---

## How to use

1. Launch **Peekaboo** (`npm run tauri:dev`)
2. **Twitch**: Get OAuth token from [twitchapps.com/tmi](https://twitchapps.com/tmi/)  
   **YouTube/Kick**: See [INTEGRATION_GUIDE.md](guide/INTEGRATION_GUIDE.md) (stubs only)
3. Set **bits/donation threshold** and choose effect
4. Configure **images/sounds** (URLs or `assets/` paths)
5. Toggle **OVERLAY** ON — transparent window appears
6. Viewers cheer/subscribe → effect fires (or click **Manual Fire** to test)

All settings auto-save to localStorage.

---

## Hotkeys

| Shortcut | Action |
|----------|--------|
| `Ctrl+Alt+Shift+K` | Emergency kill — hides overlay |
| `Ctrl+Shift+O` | Toggle overlay on/off |

---

## Project structure

```
peekaboo/
├── src-tauri/
│   ├── src/main.rs        # Rust: window management, hotkeys, click-through
│   ├── Cargo.toml
│   └── tauri.conf.json    # Window config
├── src/
│   ├── index.html         # Dashboard (control panel)
│   └── overlay.html       # Transparent overlay (effects renderer)
├── assets/
│   ├── sounds/            # Add .mp3/.wav files
│   └── images/            # Add images/GIFs
├── vite.config.js         # Vite bundler config
├── package.json
├── QUICKSTART.md          # Setup guide
└── INTEGRATION_GUIDE.md   # Platform docs
```

---

## Key features

- **Rate limiting**: 3-second cooldown prevents spam
- **Config persistence**: Settings saved automatically
- **Multi-platform stubs**: Twitch working, YouTube/Kick ready
- **Manual testing**: Fire effects without platform connection
- **Vite-powered**: Fast dev server with hot reload

---

## Security & privacy

- Peekaboo **never reads your screen or captures any video**
- The overlay is a transparent window — it only renders what you configure
- Tokens/credentials stored in localStorage (browser-level, not sent anywhere)
- All source code is here, nothing is obfuscated

---

## License

MIT — do whatever you want with it.

---

## Contributing

PRs welcome! Especially:
- YouTube integration (see [INTEGRATION_GUIDE.md](guide/INTEGRATION_GUIDE.md))
- Kick integration
- Custom effects
- Linux packaging
