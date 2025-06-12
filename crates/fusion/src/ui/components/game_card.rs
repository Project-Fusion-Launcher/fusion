use crate::ui::pages::LibraryGame;
use gpui::{prelude::FluentBuilder, *};
use ui::{DialogContext, Theme, primitives::span};

#[derive(IntoElement)]
pub struct GameCard {
    game: LibraryGame,
}

impl GameCard {
    pub fn new(game: LibraryGame) -> Self {
        Self { game }
    }

    fn open_dialog(&self, window: &mut Window, app: &mut App) {
        let game_name = self.game.name.clone();
        window.open_dialog(app, move |dialog, _, _| dialog.title(game_name.clone()));
    }
}

impl RenderOnce for GameCard {
    fn render(self, _window: &mut Window, app: &mut App) -> impl IntoElement {
        let theme = app.global::<Theme>();

        div()
            .id(self.game.id.clone())
            .group(self.game.id.clone())
            .w(rems(12.))
            .flex()
            .flex_shrink_0()
            .flex_col()
            .cursor_pointer()
            .relative()
            .child(
                div()
                    .h(rems(18.))
                    .bg(theme.colors.secondary)
                    .group_hover(self.game.id.clone(), |div| {
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
                    .when_some(self.game.cover_url.clone(), |div, _cover| {
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
                span(self.game.name.clone())
                    .w(rems(12.))
                    .relative()
                    .overflow_hidden()
                    .text_color(theme.colors.primary)
                    .mt(rems(1.))
                    .mb(rems(0.5))
                    .whitespace_nowrap()
                    .text_ellipsis()
                    .text_size(theme.text.size.md)
                    .line_height(theme.text.size.md)
                    .font_weight(FontWeight::SEMIBOLD),
            )
            .child(
                span(self.game.developer.clone())
                    .font_weight(FontWeight::LIGHT)
                    .text_size(theme.text.size.sm)
                    .line_height(theme.text.size.sm)
                    .w(rems(12.))
                    .relative()
                    .overflow_hidden()
                    .text_color(theme.colors.secondary)
                    .whitespace_nowrap()
                    .text_ellipsis(),
            )
            .on_click(move |e, window, app| self.open_dialog(window, app))
    }
}
