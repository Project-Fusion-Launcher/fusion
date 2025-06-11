use ::gpui::*;
use gpui::prelude::FluentBuilder;
use ui::Theme;

use crate::ui::{
    components::{Header, Sidebar},
    pages::{Library, Page},
};

mod components;
mod pages;

pub struct Root {
    page: AnyView,
}

impl Root {
    pub fn new(window: &mut Window, app: &mut App) -> Entity<Self> {
        app.set_global(Theme::default());
        app.set_global(Page::Library);

        let library = Library::new(window, app);

        app.new(|_cx| Self {
            page: AnyView::from(library),
        })
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let page = *cx.global::<Page>();

        div()
            .flex()
            .bg(theme.colors.background)
            .size_full()
            .font_family("Metropolis")
            .child(Sidebar::new(page))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_grow()
                    .child(Header::new(page))
                    .child(
                        div()
                            .flex()
                            .flex_grow()
                            .bg(rgb(0x000000))
                            .w_full()
                            .h_full()
                            .when(page == Page::Library, |div| div.child(self.page.clone()))
                            .when(page == Page::Downloads, |div| {
                                div.child("Downloads Content")
                            }),
                    ),
            )
    }
}
