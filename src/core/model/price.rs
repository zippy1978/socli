use rquickjs::IntoJs;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoJs)]
pub struct Price {
    pub player_slug: String,
    pub date: String,
    pub eur: String,
    pub usd: String,
}
