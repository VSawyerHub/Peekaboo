# 🚀 Quick Start Guide

## First Time Setup

### 1. Install Dependencies
```bash
npm install
```

### 2. Run Development Mode
```bash
npm run tauri:dev
```
This will:
- Start Vite dev server on port 5173
- Launch Tauri app with hot-reload
- Open dashboard window automatically

### 3. Build Production Release
```bash
npm run tauri:build
```
Installer will be in: `src-tauri/target/release/bundle/`

---

## Quick Test (No Platform Connection)

1. Launch app: `npm run tauri:dev`
2. Toggle **OVERLAY** switch ON
3. Click any **Manual Fire** button (Jumpscare, Shake, Glitch, Big Text)
4. Effect appears on screen!
5. Press **Ctrl+Alt+Shift+K** to emergency kill

---

## Hotkeys

| Hotkey | Action |
|--------|--------|
| **Ctrl+Alt+Shift+K** | Emergency kill overlay |
| **Ctrl+Shift+O** | Toggle overlay on/off |

> **Note**: ESC is NOT used to avoid conflicts with games!

---

## Platform Setup

### Twitch (Fully Working)
1. Visit [twitchapps.com/tmi](https://twitchapps.com/tmi/)
2. Click "Connect" and authorize
3. Copy the OAuth token (starts with `oauth:`)
4. Paste into dashboard
5. Enter your channel name
6. Click **Connect**

### YouTube (Stub Only)
See `INTEGRATION_GUIDE.md` for implementation details

### Kick (Stub Only)
See `INTEGRATION_GUIDE.md` for implementation details

---

## Config Persistence

All your settings are **automatically saved** to localStorage:
- Platform credentials (tokens, API keys)
- Effect configurations
- Image/sound URLs
- Thresholds and preferences

Just configure once and forget! 💾

---

## Rate Limiting

Effects have a **3-second cooldown** to prevent spam:
- Multiple bits/subs in quick succession won't flood the overlay
- Manual button spam is also limited
- Keeps overlay smooth and responsive

---

## Adding Custom Assets

### Sounds:
1. Put `.mp3` or `.wav` files in `assets/sounds/`
2. In dashboard, paste URL: `assets/sounds/your-sound.mp3`
3. Or use external URL: `https://example.com/sound.mp3`

### Images/GIFs:
1. Put files in `assets/images/`
2. In dashboard, paste URL: `assets/images/your-scare.gif`
3. Or use external URL: `https://i.imgur.com/example.gif`

---

## Troubleshooting

### Overlay not appearing?
- Check if **OVERLAY** toggle is ON
- Try **Ctrl+Shift+O** to toggle
- Press **Ctrl+Alt+Shift+K** if it's stuck

### Can't click anything?
- That's normal! Overlay is click-through
- Use dashboard to control everything

### Twitch not connecting?
- Verify OAuth token starts with `oauth:`
- Check channel name is lowercase
- Token needs `chat:read` scope

### Effects not firing?
- Check 3-second cooldown (anti-spam)
- Log shows "Effect on cooldown" if too fast

---

## Project Structure

```
peekaboo/
├── src/
│   ├── index.html         # Dashboard UI
│   └── overlay.html       # Transparent overlay
├── src-tauri/
│   ├── src/main.rs        # Rust backend
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Window config
├── assets/
│   ├── sounds/            # Audio files
│   └── images/            # Jumpscare media
├── vite.config.js         # Vite bundler config
└── package.json           # Node dependencies
```

---

## Development Tips

- **Hot reload**: Changes to HTML/CSS auto-refresh
- **Rust changes**: Requires app restart
- **Console logs**: Open DevTools in dashboard (F12)
- **Overlay logs**: Right-click overlay → Inspect (when visible)
- **Config saved**: Automatically on every input change

---

## What's New in Latest Version

✅ **Better Hotkeys**: No more ESC conflicts with games  
✅ **Auto-save**: Config persists between sessions  
✅ **Rate Limiting**: 3s cooldown prevents effect spam  
✅ **Larger Dashboard**: 820px height, resizable  
✅ **Clean Logs**: No more spam messages  

---
