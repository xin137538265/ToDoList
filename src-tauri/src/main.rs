#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use tauri::{Manager, PhysicalPosition, PhysicalSize};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState, Code, Modifiers};
use tauri_plugin_autostart::ManagerExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct WindowState {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

fn get_state_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let dir = PathBuf::from(home).join(".todo-widget");
    if !dir.exists() {
        let _ = fs::create_dir_all(&dir);
    }
    dir
}

fn load_window_state() -> Option<WindowState> {
    let path = get_state_dir().join("window.json");
    if let Ok(data) = fs::read_to_string(&path) {
        serde_json::from_str(&data).ok()
    } else {
        None
    }
}

fn save_window_state(window: &tauri::Window) {
    if let (Ok(PhysicalPosition { x, y }), Ok(PhysicalSize { width, height })) = (window.outer_position(), window.inner_size()) {
        let state = WindowState { x, y, width, height };
        let path = get_state_dir().join("window.json");
        let _ = fs::write(&path, serde_json::to_string_pretty(&state).unwrap_or_default());
    }
}

#[tauri::command]
fn toggle_autostart(app: tauri::AppHandle, enabled: bool) -> Result<String, String> {
    let manager = app.autolaunch();
    if enabled {
        let _ = manager.enable();
    } else {
        let _ = manager.disable();
    }
    Ok(format!("Autostart {}", if enabled { "enabled" } else { "disabled" }))
}

#[tauri::command]
fn start_dragging(window: tauri::Window) -> Result<(), String> {
    window.start_dragging().map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, Some(vec![])))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .setup(|app| {
            // Build tray
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("TodoList Widget")
                .on_tray_icon_event(|_tray, event| {
                    if let TrayIconEvent::Click { .. } = event {
                        let app_handle = _tray.app_handle();
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .menu(&tauri::menu::MenuBuilder::new(app)
                    .item(&tauri::menu::MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?)
                    .item(&tauri::menu::PredefinedMenuItem::separator(app)?)
                    .item(&tauri::menu::MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?)
                    .build()?)
                .on_menu_event(|app: &tauri::AppHandle, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => std::process::exit(0),
                    _ => {}
                })
                .build(app)?;

            // Register global shortcut Cmd+Shift+T
            let shortcut = Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyT);
            let global_shortcut = app.global_shortcut();
            let _ = global_shortcut.on_shortcut(shortcut, |app: &tauri::AppHandle, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    if let Some(window) = app.get_webview_window("main") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
            });

            if let Some(window) = app.get_webview_window("main") {
                #[cfg(target_os = "windows")]
                {
                    use windows::Win32::UI::WindowsAndMessaging::*;
                    use windows::Win32::Foundation::*;
                    if let Ok(hwnd) = window.hwnd() {
                        let hwnd = HWND(hwnd.0);
                        unsafe {
                            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                            SetWindowLongPtrW(
                                hwnd,
                                GWL_EXSTYLE,
                                ex_style | (WS_EX_TOOLWINDOW.0 as isize) | (WS_EX_LAYERED.0 as isize),
                            );
                        }
                    }
                }
            }

            if let Some(state) = load_window_state() {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_position(PhysicalPosition::new(state.x, state.y));
                    let _ = window.set_size(PhysicalSize::new(state.width, state.height));
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![toggle_autostart, start_dragging])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let _ = window.hide();
            }
            tauri::WindowEvent::Moved(_) | tauri::WindowEvent::Resized(_) => {
                save_window_state(window);
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}