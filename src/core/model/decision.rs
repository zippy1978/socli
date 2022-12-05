use rquickjs::FromJs;
use serde::{Deserialize, Serialize};

use super::player::Player;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, FromJs)]
pub struct Decision {
    pub action: String,
    pub player_slug: String,
    pub player_name: String,
    pub strategy: String,
    pub comment: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, FromJs)]
pub struct ScriptDecision {
    pub action: String,
    pub comment: String,
}

impl ScriptDecision {
    pub fn to_decision(&self, player: &Player, strategy_name: &str) -> Decision {
        Decision {
            action: self.action.clone(),
            player_slug: player.slug.clone(),
            player_name: player.display_name.clone(),
            strategy: strategy_name.to_string(),
            comment: self.comment.clone(),
        }
    }
}
