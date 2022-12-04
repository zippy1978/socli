use crate::core::model::{player::Player, price::Price, stats::Stats};

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        players: Vec<Player>,
        selected_player: usize,
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

    /*pub fn incr_sleep(&mut self) {
        if let Self::Initialized { counter_sleep, .. } = self {
            *counter_sleep += 1;
        }
    }

    pub fn incr_tick(&mut self) {
        if let Self::Initialized { counter_tick, .. } = self {
            *counter_tick += 1;
        }
    }

    pub fn count_sleep(&self) -> Option<u32> {
        if let Self::Initialized { counter_sleep, .. } = self {
            Some(*counter_sleep)
        } else {
            None
        }
    }

    pub fn count_tick(&self) -> Option<u64> {
        if let Self::Initialized { counter_tick, .. } = self {
            Some(*counter_tick)
        } else {
            None
        }
    }

    pub fn duration(&self) -> Option<&Duration> {
        if let Self::Initialized { duration, .. } = self {
            Some(duration)
        } else {
            None
        }
    }

    pub fn increment_delay(&mut self) {
        if let Self::Initialized { duration, .. } = self {
            // Set the duration, note that the duration is in 1s..10s
            let secs = (duration.as_secs() + 1).clamp(1, 10);
            *duration = Duration::from_secs(secs);
        }
    }

    pub fn decrement_delay(&mut self) {
        if let Self::Initialized { duration, .. } = self {
            // Set the duration, note that the duration is in 1s..10s
            let secs = (duration.as_secs() - 1).clamp(1, 10);
            *duration = Duration::from_secs(secs);
        }
    }*/
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
