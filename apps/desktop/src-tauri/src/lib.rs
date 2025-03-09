use common::database;
use managers::download::DownloadManager;
use models::config::Config;
use std::sync::{OnceLock, RwLock};
use tauri::{AppHandle, Manager};

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
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            storefronts::get_games,
            storefronts::fetch_game_versions,
            storefronts::download_game,
            storefronts::fetch_version_info,
            storefronts::launch_game,
            storefronts::uninstall_game,
            storefronts::hide_game
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
