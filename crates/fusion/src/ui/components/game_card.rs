use crate::ui::pages::LibraryGame;
use gpui::{prelude::FluentBuilder, *};
use ui::{Theme, label::Label};

#[derive(IntoElement)]
struct ImageContainer {
    text: SharedString,
    src: ImageSource,
}

impl ImageContainer {
    pub fn new(text: impl Into<SharedString>, src: impl Into<ImageSource>) -> Self {
        Self {
            text: text.into(),
            src: src.into(),
        }
    }
}

impl RenderOnce for ImageContainer {
    fn render(self, _window: &mut Window, _: &mut App) -> impl IntoElement {
        img(self.src).size(px(256.0))
    }
}

#[derive(IntoElement)]
pub struct GameCard {
    game: LibraryGame,
}

impl GameCard {
    pub fn new(game: LibraryGame) -> Self {
        Self { game }
    }
}

impl RenderOnce for GameCard {
    fn render(self, _window: &mut Window, app: &mut App) -> impl IntoElement {
        let theme = app.global::<Theme>();

        div()
            .w(rems(12.))
            .flex()
            .flex_shrink_0()
            .flex_col()
            .cursor_pointer()
            .relative()
            .group(self.game.id.clone())
            .child(
                div()
                    .h(rems(18.))
                    .bg(theme.colors.secondary)
                    .group_hover(self.game.id, |div| {
                        div.border_2()
                            .border_color(theme.colors.accent)
                            .shadow(vec![BoxShadow {
                                offset: point(px(0.), px(0.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                                color: hsla(0., 0., 1., 0.15),
                            }])
                    })
                    .relative()
                    .w_full()
                    .overflow_hidden()
                    .rounded(rems(0.75))
                    .when_some(self.game.cover_url.clone(), |div, cover| {
                        div.child(
                            img("images/capsule.webp")
                                .inset_0()
                                .absolute()
                                .h_full()
                                .rounded(rems(0.75))
                                .object_fit(ObjectFit::Cover)
                                .w_full(),
                        )
                    }),
            )
            .child(
                Label::new(self.game.name.clone())
                    .w(rems(12.))
                    .relative()
                    .overflow_hidden()
                    .text_color(theme.colors.primary)
                    .mt(rems(1.))
                    .mb(rems(0.5))
                    .whitespace_nowrap()
                    .text_ellipsis()
                    .text_left()
                    .text_size(theme.text.size.md)
                    .line_height(theme.text.size.md)
                    .font_weight(FontWeight::SEMIBOLD),
            )
            .child(
                Label::new(self.game.developer.clone())
                    .font_weight(FontWeight::LIGHT)
                    .text_size(theme.text.size.sm)
                    .line_height(theme.text.size.sm)
                    .w(rems(12.))
                    .relative()
                    .overflow_hidden()
                    .text_color(theme.colors.secondary)
                    .whitespace_nowrap()
                    .text_ellipsis()
                    .text_left(),
            )
    }
}
