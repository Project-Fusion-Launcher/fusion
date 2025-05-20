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
                        id: 843633475111337208034321273773321879117,
                        status: DownloadStatus::Queued,
                        hash: DownloadHash::Sha1(String::from(
                            "1854d657496bef58b6e81224746511500e10ec7d",
                        )),
                        compressed_size: 759559,
                        size: 1048576,
                        request: reqwest::Client::new()
                            .get("https://cloudflare.epicgamescdn.com/Builds/Org/o-x6pmn2h8elkycylakfrdw79xp2hrpv/5900d5b75cb24e4ca6ac93c30155fb72/default/ChunksV4/42/007A6880E33B53DC_3248D343425CB5B0CC921B91C5FFDA4D.chunk")
                            .header("User-Agent", "EpicGamesLauncher/11.0.1-14907503+++Portal+Release-Live Windows/10.0.19041.1.256.64bit"),
                        },
                        DownloadChunk {
                            id: 13535103561278252693245506600058062063,
                            status: DownloadStatus::Queued,
                            hash: DownloadHash::Sha1(String::from(
                                "5d2fdd80f1aedee31623c1c3f0458c61a35ff5cff7",
                            )),
                            compressed_size: 1048576,
                            size: 1048642,
                            request: reqwest::Client::new()
                                .get("https://cloudflare.epicgamescdn.com/Builds/Org/o-x6pmn2h8elkycylakfrdw79xp2hrpv/5900d5b75cb24e4ca6ac93c30155fb72/default/ChunksV4/42/F3C21297B89C6047_50ACEDD44C309695925555900375F4EF.chunk")
                                .header("User-Agent", "EpicGamesLauncher/11.0.1-14907503+++Portal+Release-Live Windows/10.0.19041.1.256.64bit"),
                        }
                    ],
                    #[cfg(windows)]
                    path: PathBuf::from("C:\\Users\\jorge\\Downloads\\test"),
                    #[cfg(unix)]
                    path: PathBuf::from("/Users/jorge/Downloads"),
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
