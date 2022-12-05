use std::{fmt::Display, path::PathBuf, fs};

use async_trait::async_trait;
use rquickjs::{Context, Function, Module, Runtime};

use crate::core::{
    model::{decision::Decision, player::Player},
    repository::error::RepoError,
};

#[derive(Debug)]
pub enum StrategyError {
    Data(String),
    Script(String),
    Config(String),
}

impl Display for StrategyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Data(msg) => write!(f, "{}", msg),
            Self::Script(msg) => write!(f, "script error: {}", msg),
            Self::Config(msg) => write!(f, "configuration error: {}", msg),
        }
    }
}

impl From<RepoError> for StrategyError {
    fn from(e: RepoError) -> Self {
        Self::Data(e.to_string())
    }
}

impl From<rquickjs::Error> for StrategyError {
    fn from(e: rquickjs::Error) -> Self {
        Self::Script(e.to_string())
    }
}

#[async_trait]
pub trait StrategyService {
    async fn run_all(&self, player: &Player) -> Result<Vec<Decision>, StrategyError>;
}

pub struct StrategyServiceImpl {
    pub strategy_dir: String,
}

impl StrategyServiceImpl {
    pub fn new(strategy_dir: &str) -> Self {
        Self {
            strategy_dir: strategy_dir.to_string(),
        }
    }

    async fn run(&self, player: &Player, code: &str) -> Result<Option<Decision>, StrategyError> {
        // If missing player prices and stats: skip eval
        if player.prices.is_empty() || player.stats.is_none() {
            return Ok(None);
        }

        let rt = Runtime::new().unwrap();
        let ctx = Context::full(&rt).unwrap();
        ctx.with(|ctx| {
            let module = Module::new(ctx, "strategy".to_string(), code)?;
            let module = module.eval()?;
            let decide: Function = module.get("decide")?;
            let res: Option<Decision> = decide.call((player.clone(), &player.slug))?;
            if let Some(decision) = res {
                Ok(Some(decision))
            } else {
                Ok(None)
            }
        })
    }
}

#[async_trait]
impl StrategyService for StrategyServiceImpl {
    async fn run_all(&self, player: &Player) -> Result<Vec<Decision>, StrategyError> {
        log::debug!("Running all strategies on {}", player.slug);

        let scripts_path = PathBuf::from(&self.strategy_dir);

        // Check if file exists
        if !scripts_path.exists() {
            return Err(StrategyError::Config(format!("failed to access directory `{}`", &self.strategy_dir)));
        }

        let mut decisions = vec![];
        let paths = fs::read_dir(scripts_path).unwrap();
        for path in paths {
            if let Ok(code) = fs::read_to_string(path.unwrap().path()) {
                if let Some(decision) = self.run(player, &code).await? {
                    decisions.push(decision);
                }
            }
        }

        Ok(decisions)
    }
}
