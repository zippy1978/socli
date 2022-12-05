use rquickjs::IntoJs;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoJs)]
pub struct Game {
    pub date: String,
    pub did_play: bool,
    pub score: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoJs)]

pub struct Stats {
    pub player_slug: String,
    pub score: i64,
    pub games: Vec<Game>,
}

impl Stats {
    pub fn last_game_scores(&self) -> Option<Vec<u64>> {
        if self.games.is_empty() {
            None
        } else {
            Some(self.games.iter().map(|g| g.score).collect())
        }
    }

    pub fn played_games_count(&self) -> Option<u64> {
        if self.games.is_empty() {
            None
        } else {
            let played: Vec<&Game> = self.games.iter().filter(|p| p.did_play).collect();
            Some(played.len() as u64)
        }
    }

    pub fn played_games_ratio(&self) -> Option<f64> {
        if self.games.is_empty() {
            None
        } else {
            let total = self.games.len() as f64;
            let played = self.played_games_count().unwrap() as f64;
            Some(played / total)
        }
    }
}
