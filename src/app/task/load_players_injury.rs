use std::sync::Arc;

use async_trait::async_trait;
use quartermaster::task::Task;

use crate::{app::App, resolve_trait, core::service::injury::InjuryService};

pub struct LoadPlayersInjuryTask {
    app: Arc<tokio::sync::Mutex<App>>,
    slugs: Vec<String>,
}
impl LoadPlayersInjuryTask {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>, slugs: Vec<String>) -> Self {
        Self { app, slugs }
    }
}

#[async_trait]
impl Task for LoadPlayersInjuryTask {
    fn name(&self) -> String {
        "load players injury".to_string()
    }

    fn id(&self) -> String {
        if self.slugs.is_empty() {
            "empty".to_string()
        } else {
            format!(
                "{}-{}",
                self.slugs.first().unwrap(),
                self.slugs.last().unwrap()
            )
        }
    }

    async fn run(&self) {
        let injury_service = resolve_trait!(InjuryService);

        // Get injuries
        match injury_service.get_injuries(&self.slugs).await {
            Ok(injuries) => {
                // Update player in app state
                let mut app = self.app.lock().await;
                app.state.merge_injuries(&self.slugs, injuries);
                // After update: run strategies
                for s in &self.slugs {
                    app.run_strategies(&s.clone()).await;
                }
            }
            Err(err) => {
                log::error!(
                    "Failed to load injury for {}: {}",
                    &self.id(),
                    err.to_string()
                )
            }
        };
    }
}
