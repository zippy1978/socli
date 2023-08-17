use std::fmt::Display;

use async_trait::async_trait;

use crate::{
    core::{
        model::injury::Injury,
        repository::{error::RepoError, injury::InjuryRepo},
    },
    resolve_trait,
};

#[derive(Debug)]
pub enum InjuryError {
    Data(String),
}

impl Display for InjuryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Data(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<RepoError> for InjuryError {
    fn from(e: RepoError) -> Self {
        Self::Data(e.to_string())
    }
}

#[async_trait]
pub trait InjuryService {
    async fn get_injuries(&self, player_slugs: &[String]) -> Result<Vec<Injury>, InjuryError>;
}

pub struct InjuryServiceImpl {}

#[async_trait]
impl InjuryService for InjuryServiceImpl {
    async fn get_injuries(&self, player_slugs: &[String]) -> Result<Vec<Injury>, InjuryError> {
        let injury_repo = resolve_trait!(InjuryRepo);

        Ok(injury_repo.get_injuries(player_slugs).await?)
    }
}
