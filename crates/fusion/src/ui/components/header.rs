use gpui::*;
use ui::{Theme, label::Label};

use crate::ui::pages::Page;

#[derive(IntoElement)]
pub struct Header {
    page: SharedString,
}

impl Header {
    pub fn new(current_page: Page) -> Self {
        Self {
            page: current_page.into(),
        }
    }
}

impl RenderOnce for Header {
    fn render(self, _window: &mut Window, app: &mut App) -> impl IntoElement {
        let theme = app.global::<Theme>();

        div()
            .flex()
            .flex_shrink_0()
            .px(rems(2.5))
            .items_center()
            .w_full()
            .h(rems(8.5))
            .child(
                Label::new(self.page)
                    .text_size(theme.text.size.xl)
                    .text_color(theme.colors.primary)
                    .font_weight(FontWeight::BOLD),
            )
    }
}
