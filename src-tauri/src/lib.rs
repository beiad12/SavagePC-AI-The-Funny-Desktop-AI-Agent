mod commands;
mod llm;
mod memory;
mod personality;
mod telemetry;
mod tools;

use commands::AppState;
use std::sync::Mutex;
use telemetry::TelemetryCollector;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let memory = memory::Memory::new().expect("failed to initialize local memory database");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            telemetry: Mutex::new(TelemetryCollector::new()),
            memory,
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_telemetry,
            commands::list_tools,
            commands::run_tool,
            commands::send_chat_message,
            commands::save_provider_config,
            commands::load_provider_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running SavagePC AI");
}
