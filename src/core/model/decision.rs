use rquickjs::FromJs;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, FromJs)]
pub struct Decision {
    pub action: String,
    pub player_slug: String,
    pub player_name: String,
    pub strategy: String,
    pub comment: String,
}
