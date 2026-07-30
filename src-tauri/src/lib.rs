mod commands;
mod llm;
mod memory;
mod monitor;
mod personality;
mod telemetry;
mod tools;

use commands::AppState;
use std::sync::{Arc, Mutex};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{Manager, WindowEvent};
use telemetry::TelemetryCollector;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let memory = Arc::new(memory::Memory::new().expect("failed to initialize local memory database"));
    let telemetry = Arc::new(Mutex::new(TelemetryCollector::new()));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .manage(AppState {
            telemetry: telemetry.clone(),
            memory: memory.clone(),
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_telemetry,
            commands::list_tools,
            commands::run_tool,
            commands::send_chat_message,
            commands::save_provider_config,
            commands::load_provider_config,
            commands::save_language,
            commands::load_language,
            commands::save_personality,
            commands::load_personality,
            commands::list_alerts,
            commands::acknowledge_alert,
        ])
        .setup(move |app| {
            let handle = app.handle().clone();
            monitor::spawn(handle.clone(), telemetry.clone(), memory.clone());

            let show_item = MenuItem::with_id(app, "show", "Show SavagePC AI", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().cloned().expect("app icon missing"))
                .menu(&tray_menu)
                .show_menu_on_left_click(true)
                .tooltip("SavagePC AI - still watching your PC")
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            // Keep the background monitor alive: closing the window hides it
            // instead of quitting the whole app. Use "Quit" in the tray menu
            // to actually exit.
            if let WindowEvent::CloseRequested { api, .. } = event {
                window.hide().ok();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running SavagePC AI");
}
