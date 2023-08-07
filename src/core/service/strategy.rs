use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use regex::Regex;
use rquickjs::{Context, Function, Runtime};

use crate::core::{
    model::{
        decision::{Decision, ScriptDecision},
        player::Player,
    },
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

    async fn run(
        &self,
        player: &Player,
        strategy_name: &str,
        code: &str,
    ) -> Result<Option<Decision>, StrategyError> {
        // If missing player prices and stats: skip eval
        if player.prices.is_empty() || player.stats.is_none() {
            return Ok(None);
        }

        let rt = Runtime::new().unwrap();
        let ctx = Context::full(&rt).unwrap();
        ctx.with(|ctx| {
            let module = ctx.compile("strategy".to_string(), code)?;
            let decide: Function = module.get("decide")?;
            let res: Option<ScriptDecision> = decide.call((player.clone(), &player.slug))?;
            if let Some(decision) = res {
                Ok(Some(decision.to_decision(&player, strategy_name)))
            } else {
                Ok(None)
            }
        })
    }

    pub(crate) fn extract_strategy_name(&self, path: &Path) -> String {
        let path_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let re = Regex::new(r"(.*)\.[^.]+$").unwrap();
        let caps = re.captures(&path_name).unwrap();
        let result = caps.get(1).map_or("", |m| m.as_str()).to_string();

        result
    }
}

#[async_trait]
impl StrategyService for StrategyServiceImpl {
    async fn run_all(&self, player: &Player) -> Result<Vec<Decision>, StrategyError> {
        log::debug!("Running all strategies on {}", player.slug);

        let scripts_path = PathBuf::from(&self.strategy_dir);

        // Check if file exists
        if !scripts_path.exists() {
            return Err(StrategyError::Config(format!(
                "failed to access directory `{}`",
                &self.strategy_dir
            )));
        }

        let mut decisions = vec![];
        let paths = fs::read_dir(scripts_path).unwrap();
        for path in paths {
            let strategy_name = self.extract_strategy_name(&path.as_ref().unwrap().path());
            if let Ok(code) = fs::read_to_string(path.unwrap().path()) {
                if let Some(decision) = self.run(player, &strategy_name, &code).await? {
                    decisions.push(decision);
                }
            }
        }

        Ok(decisions)
    }
}
