use crate::storefronts::get_games;
use database::models::{Game, GameStatus};
use gpui::*;
use gpui_tokio::Tokio;
use std::ops::Range;
use tokio::sync::oneshot;
use ui::{
    Theme,
    badge::{Badge, BadgeVariant},
    label::Label,
    tabs::{Tab, TabBar},
};

pub struct LibraryGame {
    name: String,
    status: GameStatus,
}

pub struct Library {
    active_status_tab: usize,
    games: Vec<LibraryGame>,
    filtered_game_indices: Vec<usize>,
    installed_count: usize,
}

impl Library {
    pub fn new(_window: &mut Window, app: &mut App) -> Entity<Self> {
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
                filtered_game_indices: Vec::new(),
                installed_count: 0,
            }
        })
    }

    fn set_games(&mut self, games: Vec<Game>, cx: &mut Context<Self>) {
        self.games = games
            .into_iter()
            .map(|game| {
                if game.status() == GameStatus::Installed {
                    self.installed_count += 1;
                }
                LibraryGame {
                    name: game.name().into(),
                    status: game.status(),
                }
            })
            .collect();
        self.update_filtered_games();
        cx.notify();
    }

    fn set_active_status_tab(&mut self, index: &usize, _: &mut Window, cx: &mut Context<Self>) {
        self.active_status_tab = *index;
        self.update_filtered_games();
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

    fn update_filtered_games(&mut self) {
        self.filtered_game_indices = match self.active_status_tab {
            0 => (0..self.games.len()).collect(), // All games
            1 => self
                .games
                .iter()
                .enumerate()
                .filter(|(_, game)| game.status == GameStatus::Installed)
                .map(|(index, _)| index)
                .collect(), // Installed only
            2 => self
                .games
                .iter()
                .enumerate()
                .filter(|(_, game)| game.status == GameStatus::NotInstalled)
                .map(|(index, _)| index)
                .collect(), // Not installed only
            _ => (0..self.games.len()).collect(), // Default to all games
        };
    }
}

impl Render for Library {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        // TODO: Make this more readable and configurable
        // 72px: sidebar width
        // 16px: padding on the left minus game gap
        // 40px: padding on the right
        // 192px: game width
        // 24px: game gap
        let viewport_size = window.viewport_size();
        let mut columns =
            ((viewport_size.width - px(72. + 16. + 40.)) / px(192. + 24.)).floor() as usize;
        if columns < 1 {
            columns = 1;
        }

        let primary_color = theme.colors.primary;
        let border_color = theme.colors.border;

        let not_installed_count = self.games.len() - self.installed_count;

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
                                self.create_tab_with_badge("Installed", self.installed_count, 1),
                                self.create_tab_with_badge("Not Installed", not_installed_count, 2),
                            ]),
                    ),
            )
            .child(
                uniform_list(
                    "game-library",
                    self.filtered_game_indices.len().div_ceil(columns),
                    cx.processor(move |this, range: Range<usize>, _, _| {
                        (range.start..range.end)
                            .map(|row_index: usize| {
                                let start_game_index = row_index * columns;
                                let end_game_index = (start_game_index + columns)
                                    .min(this.filtered_game_indices.len());

                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .gap(px(24.))
                                    .w_full()
                                    .children(
                                        (start_game_index..end_game_index)
                                            .map(|filtered_index| {
                                                let game_index =
                                                    this.filtered_game_indices[filtered_index];
                                                let game = &this.games[game_index];
                                                div()
                                                    .flex_shrink_0()
                                                    .w(rems(12.))
                                                    .h(rems(18.))
                                                    .mb(rems(1.5))
                                                    .p_4()
                                                    .border_1()
                                                    .border_color(border_color)
                                                    .rounded_lg()
                                                    .child(
                                                        Label::new(game.name.clone())
                                                            .text_color(primary_color),
                                                    )
                                            })
                                            .collect::<Vec<_>>(),
                                    )
                            })
                            .collect()
                    }),
                )
                .h_full()
                .w_full()
                .p(rems(2.5)),
            )
    }
}
