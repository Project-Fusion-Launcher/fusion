use gpui::{prelude::FluentBuilder, *};
use smallvec::{SmallVec, smallvec};
use std::rc::Rc;
use ui::Theme;

use crate::ui::pages::Page;

type Handler = Rc<dyn Fn(Page, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct Sidebar {
    triggers: SmallVec<[SidebarTrigger; 2]>,
    handler: Handler,
    current_page: Page,
}

impl Sidebar {
    pub fn new(handler: impl Fn(Page, &mut Window, &mut App) + 'static) -> Self {
        let handler = Rc::new(handler);
        Self {
            handler: handler.clone(),
            current_page: Page::Library,
            triggers: smallvec![
                SidebarTrigger::new(Page::Library, handler.clone()),
                SidebarTrigger::new(Page::Downloads, handler)
            ],
        }
    }

    pub fn with_current_page(mut self, page: Page) -> Self {
        self.current_page = page;
        self
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, app: &mut App) -> impl IntoElement {
        let theme = app.global::<Theme>();

        div()
            .flex()
            .flex_col()
            .flex_shrink_0()
            .bg(theme.colors.sidebar)
            .border_r_1()
            .border_color(theme.colors.border)
            .h_full()
            .w(rems(4.5))
            .items_center()
            .child(
                svg()
                    .path(theme.icons.logo.clone())
                    .text_color(theme.colors.primary)
                    .size(rems(3.))
                    .my(rems(2.75)),
            )
            .children(self.triggers.into_iter().map(|trigger| {
                let page = trigger.page;
                trigger.with_active(page == self.current_page)
            }))
    }
}

#[derive(IntoElement)]
struct SidebarTrigger {
    page: Page,
    handler: Handler,
    is_active: bool,
}

impl SidebarTrigger {
    pub fn new(page: Page, handler: Handler) -> Self {
        Self {
            page,
            handler,
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
        let handler = self.handler.clone();

        div()
            .id(ElementId::Integer(self.page as u64))
            .flex()
            .items_center()
            .justify_center()
            .h(rems(3.25))
            .cursor_pointer()
            .on_click(move |_event, window, app| {
                handler(self.page, window, app);
            })
            .child(
                svg()
                    .path(self.page.icon(theme))
                    .text_color(theme.colors.secondary)
                    .size(rems(2.)),
            )
            .when(self.is_active, |div| div.bg(theme.colors.primary))
    }
}
