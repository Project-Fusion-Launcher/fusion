use managers::download::DownloadManager;
use models::config::Config;
#[cfg(debug_assertions)]
use specta_typescript::{BigIntExportBehavior, Typescript};
use std::sync::{OnceLock, RwLock};
use tauri::{AppHandle, Manager};
use tauri_specta::{collect_commands, collect_events, Builder};

use crate::managers::database::DatabaseManager;

pub mod commands;
pub mod common;
pub mod managers;
pub mod models;
pub mod schema;
pub mod storefronts;
pub mod utils;

static APP: OnceLock<AppHandle> = OnceLock::new();

pub async fn run() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let builder = Builder::<tauri::Wry>::default()
        .commands(collect_commands![
            commands::game::get_games,
            commands::game::fetch_game_versions,
            commands::game::fetch_game_version_info,
            commands::game::download_game,
            commands::game::launch_game,
            commands::game::uninstall_game,
            commands::game::hide_game,
            managers::download::pause,
        ])
        .events(collect_events![
            models::events::GameHidden,
            models::events::GameUninstalling,
            models::events::GameUninstalled,
            models::events::GameDownloadQueued,
            models::events::GameDownloadProgress,
            models::events::GameDownloadFinished,
            models::events::GameInstalled,
        ]);

    #[cfg(debug_assertions)]
    builder
        .export(
            Typescript::default()
                .bigint(BigIntExportBehavior::Number)
                .header("// @ts-nocheck\n"),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            APP.set(app.handle().clone())
                .expect("Error setting up global app handle");

            // Initialize states/managers. The order is important, as one may depend on another.
            app.manage(DatabaseManager::init().expect("Error initializing database manager"));
            app.manage(RwLock::new(
                Config::select().expect("Error selecting config"),
            ));
            app.manage(DownloadManager::init());

            storefronts::init_storefronts();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
