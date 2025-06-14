use crate::ui::pages::LibraryGame;
use gpui::*;
use ui::{PortalContext, Theme, primitives::h_flex};

pub fn open_install_modal(game: LibraryGame, window: &mut Window, app: &mut App) {
    let theme = app.global::<Theme>();
    let rounded = theme.rounded.lg;

    window.open_modal(app, move |modal, _, _| {
        modal
            .title("Install Game")
            .description(game.name.clone())
            .child(
                h_flex().size_full().child(
                    img("images/capsule.webp")
                        .w(rems(12.))
                        .h(rems(18.))
                        .rounded(rounded)
                        .object_fit(ObjectFit::Cover),
                ),
            )
    });
}
