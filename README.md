# 👻 Peekaboo

> Interactive scare overlay for streamers — let your viewers jumpscare you in real time via bits & subs.

![License](https://img.shields.io/badge/license-MIT-blue)
![Tauri](https://img.shields.io/badge/built%20with-Tauri%20v1-orange)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS-lightgrey)

---

## What is this?

Peekaboo is a lightweight desktop app that puts a **transparent, click-through overlay** on top of everything on your screen — including your game. Viewers can trigger jumpscares, screen shakes, glitches, and more via **Twitch bits or subs**, and the streamer actually sees it happen in real time.

Unlike browser-source-only overlays, **this appears on your monitor** — so the reaction is genuine.

---

## Features

- 👻 **Jumpscare** — full-screen image/GIF + sound effect  
- 📳 **Screen shake** — camera shake feel via CSS animation  
- 📺 **Glitch effect** — RGB split + scanline corruption  
- 💬 **Big text** — huge text that slams onto screen  
- 🎮 **Twitch integration** — bits threshold + subs trigger effects  
- ⌨️ **Emergency hotkeys** — `ESC` kills overlay instantly  
- 🪟 **Click-through** — streamer can still use their PC normally  
- 🔓 **Open source** — full code, nothing hidden  

---

## Requirements

| Tool | Version |
|------|---------|
| [Rust](https://rustup.rs/) | stable (≥ 1.70) |
| [Node.js](https://nodejs.org/) | ≥ 18 |
| [Tauri CLI](https://tauri.app/v1/guides/getting-started/setup/) | v1 |

Install Tauri CLI:
```bash
npm install --save-dev @tauri-apps/cli
```

---

## Getting started

```bash
# 1. Clone
git clone https://github.com/you/peekaboo
cd peekaboo

# 2. Install JS deps
npm install

# 3. Run in dev mode
npm run dev

# 4. Build a release binary
npm run build
# → installer appears in src-tauri/target/release/bundle/
```

---

## How to use

1. Launch **Peekaboo**
2. Connect your Twitch channel (OAuth token with `chat:read` scope — get one at [twitchapps.com/tmi](https://twitchapps.com/tmi/))
3. Set your **bits threshold** and choose which effect fires
4. Configure your **jumpscare image/GIF and sound** URL
5. Toggle the **OVERLAY switch** ON — the transparent window appears
6. Viewers cheer bits or subscribe → effect fires on your screen

---

## Hotkeys

| Shortcut | Action |
|----------|--------|
| `ESC` | Emergency kill — hides overlay immediately |
| `Ctrl+Shift+P` | Toggle overlay on/off |

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
│   ├── sounds/            # Default sounds (add your own)
│   └── images/            # Default jumpscares (add your own)
└── package.json
```

---

## Adding effects

Effects are defined in `src/overlay.html` inside the `effects` object. Adding a new one:

```js
const effects = {
  // ... existing effects ...

  myeffect({ duration = 1000 }) {
    // do something wild
  }
};
```

Then add a button for it in `src/index.html` and it will appear in the dashboard automatically.

---

## Security & privacy

- Peekaboo **never reads your screen or captures any video**
- The overlay is a transparent window — it only renders what you configure
- OAuth token is stored only in memory, never written to disk
- All source code is here, nothing is obfuscated

---

## License

MIT — do whatever you want with it.

---

## Contributing

PRs welcome. If you build a cool effect, add Twitch EventSub support, or package this for Linux, open a PR!
