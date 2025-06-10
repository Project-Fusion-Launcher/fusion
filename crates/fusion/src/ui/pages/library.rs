use gpui::*;
use ui::{
    Theme,
    badge::{Badge, BadgeVariant},
    tabs::{Tab, TabBar},
};

pub struct Library {
    active_tab_index: usize,
}

impl Library {
    pub fn new(app: &mut App) -> Entity<Self> {
        app.new(|_cx| Self {
            active_tab_index: 0,
        })
    }

    fn set_active_tab(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Self>) {
        self.active_tab_index = ix;
        cx.notify();
    }

    fn create_tab_with_badge(&self, label: &'static str, count: usize, tab_index: usize) -> Tab {
        Tab::new(label).child(Badge::new().child(count.to_string()).variant(
            if self.active_tab_index == tab_index {
                BadgeVariant::Primary
            } else {
                BadgeVariant::Outline
            },
        ))
    }
}

impl Render for Library {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .flex()
            .flex_col()
            .flex_grow()
            .w_full()
            .h_full()
            .child(
                div()
                    .flex()
                    .flex_shrink_0()
                    .px(rems(2.5))
                    .h(rems(1.75))
                    .child(
                        TabBar::new("library-tabs")
                            .selected_index(self.active_tab_index)
                            .on_click(cx.listener(|this, ix, window, cx| {
                                this.set_active_tab(*ix, window, cx);
                            }))
                            .children([
                                self.create_tab_with_badge("All Games", 611, 0),
                                self.create_tab_with_badge("Installed", 1, 1),
                                self.create_tab_with_badge("Not Installed", 610, 2),
                            ]),
                    ),
            )
            .child(div().flex().flex_col().items_center().justify_center())
    }
}
