use common::database;
use managers::download::DownloadManager;
use models::config::Config;
use std::sync::{OnceLock, RwLock};
use tauri::{AppHandle, Manager};

pub mod commands;
pub mod common;
pub mod managers;
pub mod models;
pub mod schema;
pub mod storefronts;
pub mod util;

static APP: OnceLock<AppHandle> = OnceLock::new();

pub async fn run() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::game::get_games,
            commands::game::fetch_game_versions,
            commands::game::fetch_game_version_info,
            commands::game::download_game,
            commands::game::launch_game,
            commands::game::uninstall_game,
            commands::game::hide_game
        ])
        .setup(|app| {
            APP.set(app.handle().clone())
                .expect("Error setting up global app handle");

            database::init().expect("Error initializing database");
            let mut connection = database::create_connection().expect("Error creating connection");

            // Initialize states/managers. The order is important, as one may depend on another.
            app.manage(RwLock::new(
                Config::select(&mut connection).expect("Error selecting config"),
            ));
            app.manage(DownloadManager::init());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
