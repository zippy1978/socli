use std::fmt::Display;

use async_trait::async_trait;

use crate::{
    core::{
        model::price::Price,
        repository::{error::RepoError, price::PriceRepo},
    },
    resolve_trait,
};

#[derive(Debug)]
pub enum PriceError {
    Data(String),
}

impl Display for PriceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Data(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<RepoError> for PriceError {
    fn from(e: RepoError) -> Self {
        Self::Data(e.to_string())
    }
}

#[async_trait]
pub trait PriceService {
    async fn get_prices(&self, player_slug: &str) -> Result<Vec<Price>, PriceError>;
}

pub struct PriceServiceImpl {}

#[async_trait]
impl PriceService for PriceServiceImpl {
    async fn get_prices(&self, player_slug: &str) -> Result<Vec<Price>, PriceError> {
        let price_repo = resolve_trait!(PriceRepo);

        Ok(price_repo.get_prices(player_slug).await?)
    }
}
