use crate::core::model::{decision::Decision, player::Player, price::Price, stats::Stats};

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        players: Vec<Player>,
        selected_player: usize,
        decisions: Vec<Decision>,
    },
    Error(String),
}

impl AppState {
    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn update_player_selection(&mut self, selection: usize) {
        if let Self::Initialized {
            selected_player, ..
        } = self
        {
            *selected_player = selection;
        }
    }

    pub fn get_player(&self, player_slug: &str) -> Option<&Player> {
        if let Self::Initialized { players, .. } = self {
            players.iter().find(|p| p.slug == player_slug)
        } else {
            None
        }
    }

    pub fn merge_prices(&mut self, player_slug: &str, prices: Vec<Price>) {
        if let Self::Initialized { players, .. } = self {
            match players.iter_mut().find(|p| p.slug == player_slug) {
                Some(p) => p.prices = prices,
                None => (),
            }
        }
    }

    pub fn merge_stats(&mut self, stats: Vec<Stats>) {
        if let Self::Initialized { players, .. } = self {
            for s in stats {
                match players.iter_mut().find(|p| p.slug == s.player_slug) {
                    Some(p) => p.stats = Some(s),
                    None => (),
                }
            }
        }
    }

    pub fn merge_decisions(&mut self, player_slug: &str, player_decisions: Vec<Decision>) {
        if let Self::Initialized { decisions, .. } = self {
            //Filter previous decisions for player
            let mut new_decisions: Vec<Decision> = decisions
                .iter()
                .filter(|d| d.player_slug != player_slug)
                .map(|d| d.clone())
                .collect();

            // Add new ones
            new_decisions.append(&mut player_decisions.clone());

            *decisions = new_decisions;
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
