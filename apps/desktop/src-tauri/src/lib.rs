use managers::{config::ConfigManager, database::DatabaseManager};
use std::sync::OnceLock;
use tauri::{AppHandle, Manager};

pub mod managers;
pub mod models;
pub mod schema;
pub mod storefronts;

/// A [`OnceLock`](OnceLock) containing a [`tauri`](tauri) [`AppHandle`](AppHandle) for easy access.
static APP: OnceLock<AppHandle> = OnceLock::new();

pub async fn run() {
    // Shares the current tokio runtime with tauri.
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .invoke_handler(tauri::generate_handler![storefronts::get_games,])
        .setup(|app| {
            APP.set(app.handle().clone())
                .expect("Error setting up global app handle");

            app.manage(DatabaseManager::new());
            app.manage(ConfigManager::new());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
