use ::gpui::*;
use gpui::prelude::FluentBuilder;
use ui::Theme;

use crate::ui::{components::Sidebar, pages::Page};

mod components;
mod pages;

pub struct TempState {
    pub page: Page,
}

impl TempState {
    pub fn new() -> Self {
        Self {
            page: Page::Library,
        }
    }
}

impl Global for TempState {}

pub struct Root {}

impl Root {
    pub fn new(app: &mut App) -> Entity<Self> {
        app.set_global(Theme::default());
        app.set_global(TempState::new());

        app.new(|_cx| Self {})
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let page = cx.global::<TempState>().page;

        div()
            .flex()
            .bg(theme.colors.background)
            .size_full()
            .child(
                Sidebar::new(|new_page, _, app| {
                    app.update_global::<TempState, _>(|temp_state, _app| {
                        temp_state.page = new_page;
                    });
                })
                .with_current_page(page),
            )
            .when(page == Page::Library, |div| div.bg(rgb(0xff0000)))
            .when(page == Page::Downloads, |div| div.bg(rgb(0x0000ff)))
    }
}
