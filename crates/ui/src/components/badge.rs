use crate::Theme;
use gpui::*;

#[derive(Clone, Copy)]
pub enum BadgeVariant {
    Primary,
    Outline,
}

#[derive(IntoElement)]
pub struct Badge {
    base: Div,
    variant: BadgeVariant,
}

impl Badge {
    pub fn new() -> Self {
        Self {
            base: div(),
            variant: BadgeVariant::Primary,
        }
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl Default for Badge {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Badge {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, app: &mut App) -> impl IntoElement {
        let theme = app.global::<Theme>();

        let element = match self.variant {
            BadgeVariant::Primary => self
                .base
                .border_color(theme.colors.transparent)
                .bg(theme.colors.primary)
                .text_color(theme.colors.primary_foreground),
            BadgeVariant::Outline => self
                .base
                .border_color(theme.colors.border)
                .text_color(theme.colors.secondary),
        };

        element
            .flex()
            .border_1()
            .text_size(theme.text.size.sm)
            .h(rems(1.75))
            .px(rems(0.5))
            .items_center()
            .justify_center()
            .rounded(theme.rounded.md)
    }
}
