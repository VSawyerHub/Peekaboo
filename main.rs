// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    AppHandle, Builder, GlobalShortcutManager, Manager, 
    PhysicalSize, WindowBuilder, WindowUrl,
};

/// Fired from the frontend dashboard to trigger an effect on the overlay
#[tauri::command]
fn trigger_effect(app: AppHandle, effect: String, payload: serde_json::Value) {
    if let Some(overlay) = app.get_window("overlay") {
        overlay
            .emit("effect", serde_json::json!({ "type": effect, "payload": payload }))
            .ok();
    }
}

/// Show or hide the overlay window
#[tauri::command]
fn set_overlay_visible(app: AppHandle, visible: bool) {
    if let Some(overlay) = app.get_window("overlay") {
        if visible {
            overlay.show().ok();
        } else {
            overlay.hide().ok();
        }
    }
}

fn main() {
    Builder::default()
        .setup(|app| {
            let handle = app.handle();

            // ── Overlay window ───────────────────────────────────────────────
            // Transparent, always-on-top, click-through, no decorations
            let overlay = WindowBuilder::new(
                app,
                "overlay",
                WindowUrl::App("overlay.html".into()),
            )
            .title("Peekaboo Overlay")
            .fullscreen(true)
            .decorations(false)
            .transparent(true)
            .always_on_top(true)
            .skip_taskbar(true)
            // Start hidden — only shown when streamer enables it
            .visible(false)
            .build()?;

            // Make the window click-through (mouse events pass to apps below)
            #[cfg(target_os = "windows")]
            {
                use windows::Win32::Foundation::HWND;
                use windows::Win32::UI::WindowsAndMessaging::{
                    GetWindowLongW, SetWindowLongW, GWL_EXSTYLE,
                    WS_EX_LAYERED, WS_EX_TRANSPARENT,
                };
                let hwnd = HWND(overlay.hwnd().unwrap().0);
                unsafe {
                    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                    SetWindowLongW(
                        hwnd,
                        GWL_EXSTYLE,
                        ex_style | WS_EX_LAYERED.0 as i32 | WS_EX_TRANSPARENT.0 as i32,
                    );
                }
            }

            #[cfg(target_os = "macos")]
            {
                use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior};
                use cocoa::base::id;
                let ns_window = overlay.ns_window().unwrap() as id;
                unsafe {
                    ns_window.setIgnoresMouseEvents_(true);
                    // Float above fullscreen apps (e.g. games)
                    let behavior = ns_window.collectionBehavior()
                        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary;
                    ns_window.setCollectionBehavior_(behavior);
                }
            }

            // ── Dashboard window ─────────────────────────────────────────────
            WindowBuilder::new(
                app,
                "dashboard",
                WindowUrl::App("index.html".into()),
            )
            .title("Peekaboo — Dashboard")
            .inner_size(420.0, 680.0)
            .resizable(false)
            .decorations(true)
            .build()?;

            // ── Global hotkeys ───────────────────────────────────────────────
            let mut shortcuts = handle.global_shortcut_manager();

            // ESC = emergency kill: hide overlay immediately
            let h1 = handle.clone();
            shortcuts.register("Escape", move || {
                if let Some(w) = h1.get_window("overlay") {
                    w.hide().ok();
                    w.emit("kill", ()).ok();
                }
            })?;

            // Ctrl+Shift+P = toggle overlay visibility (streamer convenience)
            let h2 = handle.clone();
            shortcuts.register("Ctrl+Shift+P", move || {
                if let Some(w) = h2.get_window("overlay") {
                    if w.is_visible().unwrap_or(false) {
                        w.hide().ok();
                    } else {
                        w.show().ok();
                    }
                }
            })?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![trigger_effect, set_overlay_visible])
        .run(tauri::generate_context!())
        .expect("error while running Peekaboo");
}
