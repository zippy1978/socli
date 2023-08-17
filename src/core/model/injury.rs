use rquickjs::IntoJs;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoJs)]
pub struct Injury {
    pub player_slug: String,
    pub date: String,
    pub update_date: Option<String>,
    pub description: String,
    pub comment: String,
}
