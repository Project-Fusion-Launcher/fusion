use gpui::{prelude::FluentBuilder, *};
use ui::{Theme, label::Label};

use crate::ui::pages::Page;

#[derive(IntoElement)]
pub struct Sidebar {
    current_page: Page,
}

impl Sidebar {
    pub fn new(current_page: Page) -> Self {
        Self { current_page }
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, app: &mut App) -> impl IntoElement {
        let theme = app.global::<Theme>();

        div()
            .relative()
            .flex()
            .flex_col()
            .flex_shrink_0()
            .bg(theme.colors.sidebar)
            .border_r_1()
            .border_color(theme.colors.border)
            .h_full()
            .pb(rems(0.25))
            .w(rems(4.5))
            .items_center()
            .child(
                svg()
                    .path(theme.icons.logo.clone())
                    .text_color(theme.colors.primary)
                    .size(rems(3.))
                    .my(rems(2.75)),
            )
            .child(
                div()
                    .relative()
                    .flex()
                    .flex_col()
                    .flex_grow()
                    .items_center()
                    .w_full()
                    .children([
                        SidebarTrigger::new(Page::Library)
                            .with_active(self.current_page == Page::Library),
                        SidebarTrigger::new(Page::Downloads)
                            .with_active(self.current_page == Page::Downloads),
                    ])
                    .child(
                        div()
                            .absolute()
                            .h(rems(3.25))
                            .border_r_2()
                            .border_color(theme.colors.accent)
                            .w_full()
                            .top(rems(3.25 * self.current_page as u8 as f32)),
                    ),
            )
            .child(
                Label::new("0.0.1")
                    .font_weight(FontWeight::LIGHT)
                    .text_size(theme.text.size.xs)
                    .text_center(),
            )
            .text_color(theme.colors.secondary)
    }
}

#[derive(IntoElement)]
struct SidebarTrigger {
    page: Page,
    is_active: bool,
}

impl SidebarTrigger {
    pub fn new(page: Page) -> Self {
        Self {
            page,
            is_active: false,
        }
    }

    pub fn with_active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }
}

impl RenderOnce for SidebarTrigger {
    fn render(self, _window: &mut Window, app: &mut App) -> impl IntoElement {
        let theme = app.global::<Theme>();

        div()
            .id(ElementId::Integer(self.page as u64))
            .flex()
            .items_center()
            .justify_center()
            .h(rems(3.25))
            .cursor_pointer()
            .on_click(move |_event, _window, app| {
                app.update_global::<Page, _>(|page, _app| {
                    *page = self.page;
                });
            })
            .child(
                svg()
                    .path(self.page.icon(theme))
                    .text_color(theme.colors.secondary)
                    .size(rems(2.))
                    .when(self.is_active, |svg| {
                        svg.text_color(theme.colors.primary)
                            .with_transformation(Transformation::scale(Size::new(1.05, 1.05)))
                    }),
            )
    }
}
