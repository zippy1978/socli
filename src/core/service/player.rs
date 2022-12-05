use std::fmt::Display;

use async_trait::async_trait;
use serde_json::{from_value, to_value};

use crate::{
    core::{
        model::player::{Player},
        repository::{error::RepoError, player::PlayerRepo, storage::StorageRepo},
    },
    resolve_trait,
};

#[derive(Debug)]
pub enum PlayerError {
    Data(String),
}

impl Display for PlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Data(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<RepoError> for PlayerError {
    fn from(e: RepoError) -> Self {
        Self::Data(e.to_string())
    }
}

#[async_trait]
pub trait PlayerService {
    async fn get_players(&self) -> Result<Vec<Player>, PlayerError>;
}

pub struct PlayerServiceImpl {}

#[async_trait]
impl PlayerService for PlayerServiceImpl {
    async fn get_players(&self) -> Result<Vec<Player>, PlayerError> {
        let player_repo = resolve_trait!(PlayerRepo);
        let storage_repo = resolve_trait!(StorageRepo);

        // Check if player list is already stored
        // Otherwise call API
        match storage_repo.get_collection("players").await? {
            Some(v) => match from_value(v) {
                Ok(players) => Ok(players),
                Err(err) => Err(PlayerError::Data(err.to_string())),
            },
            None => {
                // Limit to 450 players
                let players = player_repo.get_players(450).await?;
                // Store
                match to_value(&players) {
                    Ok(v) => {
                        storage_repo.set_collection("players", &v).await?;
                        Ok(players)
                    }
                    Err(err) => Err(PlayerError::Data(err.to_string())),
                }
            }
        }
    }
}
