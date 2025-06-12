use crate::{path::PathResolver, ui::Root};
use ::ui::ContextProvider;
use anyhow::Result;
use assets::Assets;
use database::{ConnectionPool, models::Config};
use gpui::*;
use std::fs;

mod path;
mod storefronts;
#[path = "ui/root.rs"]
mod ui;

pub const APP_ID: &str = "fusion";
pub const APP_NAME: &str = "Fusion";
pub const DB_NAME: &str = "fusion.db";

fn main() -> Result<()> {
    let app_data_dir = PathResolver::app_data_dir();
    fs::create_dir_all(&app_data_dir)?;

    let pool = ConnectionPool::new(app_data_dir.join(DB_NAME))?;
    pool.run_pending_migrations()?;

    let config = Config::init()?;

    let options = WindowOptions {
        app_id: Some(APP_ID.into()),
        titlebar: Some(TitlebarOptions {
            title: Some(APP_NAME.into()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let application = Application::new().with_assets(Assets);
    application.run(|app| {
        Assets.load_fonts(app).unwrap();
        gpui_tokio::init(app);

        app.set_global(pool);
        app.set_global(config);

        //storefronts::init(app).unwrap();

        app.open_window(options, |window, app| {
            let root = Root::new(window, app);
            app.new(|cx| ContextProvider::new(root.into(), window, cx))
        })
        .unwrap();
    });

    Ok(())
}
