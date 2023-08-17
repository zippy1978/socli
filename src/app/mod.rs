use strum::IntoEnumIterator;

use crate::core::model::player::Player;

use self::{
    action::{Action, Actions},
    input::key::Key,
    io::IoEvent,
    state::{AppState, Panel},
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
            decisions: vec![],
            selected_panel: Panel::Players,
            selected_decision: 0,
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
                Action::Backspace => self.clear_decisions(),
                Action::Tab => self.next_panel_selection(),
            }
        } else {
            log::warn!("No action bound to {}", key);
            AppReturn::Continue
        }
    }

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

    pub async fn initialize(&mut self, players: Vec<Player>) {
        // Update contextual actions
        self.actions = vec![
            Action::Quit,
            Action::Down,
            Action::Up,
            Action::PageUp,
            Action::PageDown,
            Action::Backspace,
            Action::Tab,
        ]
        .into();
        self.state = AppState::Initialized {
            players,
            selected_player: 0,
            decisions: vec![],
            selected_panel: Panel::Players,
            selected_decision: 0,
        };
        self.refresh_player_prices(0, false).await;
    }

    pub async fn refresh_player_prices(&mut self, index: usize, force: bool) {
        // Trigger player details load / update
        if let AppState::Initialized { players, .. } = &self.state {
            match players.get(index) {
                Some(p) => {
                    if force || (!force && p.prices.is_empty()) {
                        self.dispatch(IoEvent::LoadPlayerPrices(p.slug.to_string()))
                            .await
                    }
                }
                None => (),
            }
        }
    }

    pub fn clear_decisions(&mut self) -> AppReturn {
        if let AppState::Initialized { decisions, .. } = &mut self.state {
            decisions.clear();
        }
        AppReturn::Continue
    }

    pub fn next_panel_selection(&mut self) -> AppReturn {
        if let AppState::Initialized { selected_panel, .. } = &mut self.state {
            let len = Panel::iter().len();
            let current_pos = Panel::iter()
                .enumerate()
                .find(|(_, p)| p == selected_panel)
                .map(|(i, _)| i);

            if let Some(pos) = current_pos {
                let next_pos = (pos + 1) % len;
                for (i, p) in Panel::iter().enumerate() {
                    if next_pos == i {
                        *selected_panel = p;
                    }
                }
            }
        }

        AppReturn::Continue
    }

    pub async fn refresh_players_stats(&mut self, player_slugs: &[String]) {
        // Trigger players stats load / update
        if let AppState::Initialized { .. } = &self.state {
            self.dispatch(IoEvent::LoadPlayersStats(player_slugs.to_vec()))
                .await;
        }
    }

    pub async fn refresh_players_injury(&mut self, player_slugs: &[String]) {
        // Trigger players injury load / update
        if let AppState::Initialized { .. } = &self.state {
            self.dispatch(IoEvent::LoadPlayersInjury(player_slugs.to_vec()))
                .await;
        }
    }

    pub async fn run_strategies(&mut self, player_slug: &str) {
        if let AppState::Initialized { .. } = &self.state {
            self.dispatch(IoEvent::RunStrategies(player_slug.to_string()))
                .await;
        }
    }

    pub async fn go_up(&mut self, step: usize) -> AppReturn {
        if let AppState::Initialized {
            selected_player,
            selected_panel,
            selected_decision,
            ..
        } = &self.state
        {
            let selected = *match selected_panel {
                Panel::Players => selected_player,
                Panel::Decisions => selected_decision,
                Panel::Logs => &0,
                Panel::Player => &0,
            };

            let selection = if selected > step { selected - step } else { 0 };
            self.update_selection(selection, selected_panel.clone())
                .await;
        }

        AppReturn::Continue
    }

    pub async fn go_down(&mut self, step: usize) -> AppReturn {
        if let AppState::Initialized {
            players,
            decisions,
            selected_player,
            selected_panel,
            selected_decision,
            ..
        } = &self.state
        {
            let (len, selected) = match selected_panel {
                Panel::Players => (players.len(), *selected_player),
                Panel::Decisions => (decisions.len(), *selected_decision),
                Panel::Logs => (0, 0),
                Panel::Player => (0, 0),
            };

            if len > 0 {
                let selection = if (selected + step) < len - 1 {
                    selected + step
                } else {
                    len - 1
                };
                self.update_selection(selection, selected_panel.clone())
                    .await;
            }
        }

        AppReturn::Continue
    }

    pub async fn update_selection(&mut self, selection: usize, panel: Panel) {
        self.state.update_selection(selection, panel);

        if matches!(panel, Panel::Players) {
            self.refresh_player_prices(selection, false).await;
        }
    }

    pub fn error(&mut self, msg: String) {
        self.state = AppState::Error(msg)
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }
}
