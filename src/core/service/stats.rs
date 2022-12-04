use std::fmt::Display;

use async_trait::async_trait;

use crate::{
    core::{
        model::{stats::Stats},
        repository::{error::RepoError, stats::StatsRepo},
    },
    resolve_trait,
};

#[derive(Debug)]
pub enum StatsError {
    Data(String),
}

impl Display for StatsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Data(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<RepoError> for StatsError {
    fn from(e: RepoError) -> Self {
        Self::Data(e.to_string())
    }
}

#[async_trait]
pub trait StatsService {
    async fn get_stats(&self, player_slugs: &[String]) -> Result<Vec<Stats>, StatsError>;
}

pub struct StatsServiceImpl {}

#[async_trait]
impl StatsService for StatsServiceImpl {
    async fn get_stats(&self, player_slugs: &[String]) -> Result<Vec<Stats>, StatsError> {
        let stats_repo = resolve_trait!(StatsRepo);

        Ok(stats_repo.get_stats(player_slugs).await?)
    }
}
