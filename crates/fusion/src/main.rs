use crate::{path::PathResolver, ui::Root};
use anyhow::Result;
use assets::Assets;
use gpui::*;
use std::fs;

pub const APP_ID: &str = "fusion";
pub const APP_NAME: &str = "Fusion";

mod path;
#[path = "ui/root.rs"]
mod ui;

fn main() -> Result<()> {
    fs::create_dir_all(PathResolver::app_data_dir())?;

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

        app.open_window(options, |_window, app| Root::new(app))
            .unwrap();
    });

    Ok(())
}
