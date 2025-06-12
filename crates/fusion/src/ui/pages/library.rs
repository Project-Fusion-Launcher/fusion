use crate::{storefronts::get_games, ui::components::GameCard};
use database::models::{Game, GameStatus};
use gpui::*;
use gpui_tokio::Tokio;
use std::ops::Range;
use tokio::sync::oneshot;
use ui::{
    components::{Badge, BadgeVariant, Tab, TabBar},
    primitives::{h_flex, v_flex},
};

#[derive(Clone)]
pub struct LibraryGame {
    pub id: SharedString,
    pub name: SharedString,
    pub developer: SharedString,
    pub status: GameStatus,
    pub cover_url: Option<SharedUri>,
}

pub struct Library {
    active_status_tab: usize,
    games: Vec<LibraryGame>,
    filtered_game_indices: Vec<usize>,
    installed_count: usize,

    columns: usize,
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

            cx.observe_window_bounds(window, Self::update_columns)
                .detach();

            Self {
                active_status_tab: 0,
                games: Vec::new(),
                filtered_game_indices: Vec::new(),
                installed_count: 0,
                columns: Self::compute_colums(window, cx),
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
                    id: game.id().to_owned().into(),
                    name: game.name().to_owned().into(),
                    developer: game.developer().unwrap_or("Unknown").to_owned().into(),
                    status: game.status(),
                    cover_url: game.cover_url().map(|url| url.to_owned().into()),
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

    fn compute_colums(window: &mut Window, _cx: &mut Context<Self>) -> usize {
        // TODO: Make this more readable and configurable
        // 72px: sidebar width
        // 16px: padding on the left minus game gap
        // 40px: padding on the right
        // 192px: game width
        // 24px: game gap
        let viewport_size = window.viewport_size();
        let columns =
            ((viewport_size.width - px(72. + 16. + 40.)) / px(192. + 24.)).floor() as usize;
        columns.max(1)
    }

    fn update_columns(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.columns = Self::compute_colums(window, cx);
        cx.notify();
    }
}

impl Render for Library {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let not_installed_count = self.games.len() - self.installed_count;
        let columns = self.columns;

        v_flex()
            .size_full()
            .child(
                h_flex().flex_shrink_0().px(rems(2.5)).h(rems(1.75)).child(
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

                                h_flex()
                                    .justify_between()
                                    .gap(px(24.))
                                    .pb(rems(2.5))
                                    .w_full()
                                    .children(
                                        (start_game_index..end_game_index)
                                            .map(|filtered_index| {
                                                let game_index =
                                                    this.filtered_game_indices[filtered_index];
                                                let game = &this.games[game_index];
                                                GameCard::new(game.clone())
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
