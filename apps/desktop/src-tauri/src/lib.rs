use common::database;
use managers::{download::DownloadManager, download_new::DownloadManager2};
use models::{
    config::Config,
    download::{Download, DownloadChunk, DownloadFile, DownloadHash, DownloadStatus},
    game::GameSource,
};
use reqwest::RequestBuilder;
use std::{
    path::PathBuf,
    sync::{OnceLock, RwLock},
};
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

            tauri::async_runtime::spawn(async {
                storefronts::init_storefronts()
                    .await
                    .expect("Error initializing storefronts");

                let download_manager_2 = DownloadManager2::init();

                download_manager_2.enqueue_download(Download {
                    files: vec![],
                    chunks: vec![DownloadChunk {
                        id: 263186378597728493026504637363234853699,
                        status: DownloadStatus::Queued,
                        hash: DownloadHash::Sha1(String::from(
                            "5a6abd08f571fb662aff3e8cccf5848cd1858f40",
                        )),
                        compressed_size: 759559,
                        size: 1048576,
                        request: reqwest::Client::new()
                            .get("https://example.com")
                            .header("User-Agent", "EpicGamesLauncher/11.0.1-14907503+++Portal+Release-Live Windows/10.0.19041.1.256.64bit"),
                    }],
                    path: PathBuf::from("C:\\Users\\jorge\\Downloads\\test"),
                    game_id: "test".to_string(),
                    game_source: GameSource::EpicGames,
                    game_title: "test".to_string(),
                });

                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                download_manager_2.pause_download();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
