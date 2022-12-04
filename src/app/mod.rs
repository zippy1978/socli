use crate::core::model::player::Player;

use self::{
    action::{Action, Actions},
    input::key::Key,
    io::IoEvent,
    state::AppState,
};

pub mod action;
pub mod input;
pub mod io;
pub mod state;
pub mod task;
pub mod ui;
pub mod widget;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

/// The main application, containing the state
pub struct App {
    /// We could dispatch an IO event
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    /// Contextual actions
    actions: Actions,
    /// State
    is_loading: bool,
    state: AppState,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IoEvent>) -> Self {
        let actions = vec![Action::Quit].into();
        let is_loading = false;
        let state = AppState::Initialized {
            players: vec![],
            selected_player: 0,
        };

        Self {
            io_tx,
            actions,
            state,
            is_loading,
        }
    }

    /// Handle a user action
    pub async fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            log::debug!("Run action [{:?}]", action);
            match action {
                Action::Quit => AppReturn::Exit,
                Action::Up => self.go_up(1).await,
                Action::Down => self.go_down(1).await,
                Action::PageUp => self.go_up(20).await,
                Action::PageDown => self.go_down(20).await,
            }
        } else {
            log::warn!("No action accociated to {}", key);
            AppReturn::Continue
        }
    }

    /// We could update the app or dispatch event on tick
    /*pub async fn update_on_tick(&mut self) -> AppReturn {
        // here we just increment a counter
        self.state.incr_tick();
        AppReturn::Continue
    }*/

    /// Send a network event to the IO thread
    pub async fn dispatch(&mut self, action: IoEvent) {
        // `is_loading` will be set to false again after the async action has finished in io/handler.rs
        self.is_loading = true;
        if let Err(e) = self.io_tx.send(action).await {
            self.is_loading = false;
            log::error!("Error from dispatch {}", e);
        };
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }
    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub async fn initialized(&mut self, players: Vec<Player>) {
        // Update contextual actions
        self.actions = vec![
            Action::Quit,
            Action::Down,
            Action::Up,
            Action::PageUp,
            Action::PageDown,
        ]
        .into();
        self.state = AppState::Initialized {
            players,
            selected_player: 0,
        };
        self.refresh_player_prices(0, false).await;
    }

    pub async fn refresh_player_prices(&mut self, index: usize, force: bool) {
        // Trigger player details load / update
        if let AppState::Initialized { players, .. } = &self.state {
            match players.get(index) {
                Some(p) => {
                    if force || (!force && p.prices.is_empty()) {
                        self.dispatch(IoEvent::LoadPlayerDetails(p.slug.to_string()))
                            .await
                    }
                }
                None => (),
            }
        }
    }

    pub async fn refresh_players_stats(&mut self, player_slugs: &[String]) {
        // Trigger players stats load / update
        if let AppState::Initialized {  .. } = &self.state {
            self.dispatch(IoEvent::LoadPlayersStats(player_slugs.to_vec())).await;
        }
    }

    pub async fn go_up(&mut self, step: usize) -> AppReturn {
        match &self.state {
            AppState::Initialized {
                players: _,
                selected_player,
            } => {
                let selection = if *selected_player > step {
                    *selected_player - step
                } else {
                    0
                };
                self.update_player_selection(selection).await;
            }
            _ => (),
        }
        AppReturn::Continue
    }

    pub async fn go_down(&mut self, step: usize) -> AppReturn {
        match &self.state {
            AppState::Initialized {
                players,
                selected_player,
            } => {
                let selection = if (*selected_player + step) < players.len() - 1 {
                    *selected_player + step
                } else {
                    players.len() - 1
                };
                self.update_player_selection(selection).await;
            }
            _ => (),
        }
        AppReturn::Continue
    }

    pub async fn update_player_selection(&mut self, selection: usize) {
        self.state.update_player_selection(selection);

        self.refresh_player_prices(selection, false).await;
    }

    pub fn error(&mut self, msg: String) {
        self.state = AppState::Error(msg)
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }
}
