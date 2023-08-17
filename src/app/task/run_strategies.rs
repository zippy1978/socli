use std::sync::Arc;

use async_trait::async_trait;
use quartermaster::task::Task;

use crate::{
    app::App,
    core::{model::player::Player, service::strategy::StrategyService},
    resolve_trait,
};

pub struct RunStrategiesTask {
    app: Arc<tokio::sync::Mutex<App>>,
    player: Player,
}
impl RunStrategiesTask {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>, player: Player) -> Self {
        Self { app, player }
    }
}

#[async_trait]
impl Task for RunStrategiesTask {
    fn name(&self) -> String {
        "run strategies".to_string()
    }

    fn id(&self) -> String {
        self.player.slug.to_string()
    }

    async fn run(&self) {
        let strategy_service = resolve_trait!(StrategyService);

        match strategy_service.run_all(&self.player).await {
            Ok(decisions) => {
                let mut app = self.app.lock().await;
                app.state.merge_decisions(&self.player.slug, decisions);
            }
            Err(e) => {
                log::error!(
                    "Failed to run strategies on {}: {}",
                    self.player.slug,
                    e.to_string()
                )
            }
        }
    }
}
