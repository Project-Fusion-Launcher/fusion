use common::database;
use managers::{config::ConfigManager, download::DownloadManager};
use std::sync::OnceLock;
use tauri::{AppHandle, Manager};

pub mod common;
pub mod managers;
pub mod models;
pub mod schema;
pub mod storefronts;

/// A [`OnceLock`](OnceLock) containing a [`tauri`](tauri) [`AppHandle`](AppHandle) for easy access.
static APP: OnceLock<AppHandle> = OnceLock::new();

pub async fn run() {
    // Share the current tokio runtime with tauri.
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            storefronts::get_games,
            storefronts::fetch_game_versions,
            storefronts::download_game,
            storefronts::fetch_version_info,
        ])
        .setup(|app| {
            APP.set(app.handle().clone())
                .expect("Error setting up global app handle");

            database::init();

            // Initialize states/managers. The order is important, as one may depend on another.
            app.manage(ConfigManager::init());
            app.manage(DownloadManager::init());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
