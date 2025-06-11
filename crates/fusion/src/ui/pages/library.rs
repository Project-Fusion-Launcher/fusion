use std::ops::Range;

use database::models::Game;
use gpui::*;
use gpui_tokio::Tokio;
use tokio::sync::oneshot;
use ui::{
    Theme,
    badge::{Badge, BadgeVariant},
    label::Label,
    tabs::{Tab, TabBar},
};

use crate::storefronts::get_games;

pub struct Library {
    active_status_tab: usize,
    games: Vec<Game>,
    rows: usize,
    scroll: UniformListScrollHandle,
}

impl Library {
    pub fn new(window: &mut Window, app: &mut App) -> Entity<Self> {
        let (tx, rx) = oneshot::channel();

        Tokio::spawn(app, async {
            let games = get_games(false).await.unwrap();
            tx.send(games).unwrap();
        })
        .detach();

        app.new(|cx| {
            cx.spawn(async |this, app| {
                match rx.await {
                    Ok(games) => {
                        this.update(app, |library: &mut Library, cx| {
                            library.set_games(games, cx)
                        })
                        .unwrap();
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch games: {:?}", e);
                    }
                };
            })
            .detach();

            Self {
                active_status_tab: 0,
                games: Vec::new(),
                scroll: UniformListScrollHandle::default(),
                rows: 0,
            }
        })
    }

    fn set_games(&mut self, games: Vec<Game>, cx: &mut Context<Self>) {
        self.games = games;
        cx.notify();
    }

    fn set_active_status_tab(&mut self, index: &usize, _: &mut Window, cx: &mut Context<Self>) {
        self.active_status_tab = *index;
        cx.notify();
    }

    fn create_tab_with_badge(&self, label: &'static str, count: usize, tab_index: usize) -> Tab {
        Tab::new(label).child(Badge::new().child(count.to_string()).variant(
            if self.active_status_tab == tab_index {
                BadgeVariant::Primary
            } else {
                BadgeVariant::Outline
            },
        ))
    }
}

impl Render for Library {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let size = window.viewport_size();

        let mut columns = ((size.width - px(72. + 16. + 40.)) / px(192. + 24.)).floor() as usize;
        if columns < 1 {
            columns = 1;
        }

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
                            .selected_index(self.active_status_tab)
                            .on_click(cx.listener(Self::set_active_status_tab))
                            .children([
                                self.create_tab_with_badge("All Games", self.games.len(), 0),
                                self.create_tab_with_badge("Installed", 0, 1),
                                self.create_tab_with_badge("Not Installed", self.games.len(), 2),
                            ]),
                    ),
            )
            .child(
                uniform_list(
                    "game-library",
                    self.games.len().div_ceil(columns),
                    cx.processor(move |this, range: Range<usize>, _, cx| {
                        let theme = cx.global::<Theme>();

                        (range.start..range.end)
                            .map(|row_index: usize| {
                                let start_game_index = row_index * columns;
                                let end_game_index =
                                    (start_game_index + columns).min(this.games.len());

                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .gap(px(24.))
                                    .w_full()
                                    .children(
                                        (start_game_index..end_game_index)
                                            .map(|game_index| {
                                                let game = &this.games[game_index];
                                                div()
                                                    .flex_shrink_0()
                                                    .w(rems(12.))
                                                    .h(rems(18.))
                                                    .mb(rems(1.5))
                                                    .p_4()
                                                    .border_1()
                                                    .border_color(theme.colors.border)
                                                    .rounded_lg()
                                                    .child(
                                                        Label::new(game.name().to_owned())
                                                            .text_color(theme.colors.primary),
                                                    )
                                            })
                                            .collect::<Vec<_>>(),
                                    )
                            })
                            .collect::<Vec<_>>()
                    }),
                )
                .h_full()
                .w_full()
                .p(rems(2.5))
                .track_scroll(self.scroll.clone()),
            )
    }
}
