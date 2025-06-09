use ::ui::Assets;
use gpui::*;

use crate::ui::Root;

#[path = "ui/root.rs"]
mod ui;

fn main() {
    Application::new().with_assets(Assets).run(|app| {
        app.open_window(WindowOptions::default(), |_window, app| Root::new(app))
            .unwrap();
    });
}
