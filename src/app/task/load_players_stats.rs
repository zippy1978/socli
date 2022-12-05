use std::sync::Arc;

use async_trait::async_trait;
use quartermaster::task::Task;

use crate::{app::App, core::service::{stats::StatsService}, resolve_trait};

pub struct LoadPlayersStatsTask {
    app: Arc<tokio::sync::Mutex<App>>,
    slugs: Vec<String>,
}
impl LoadPlayersStatsTask {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>, slugs: Vec<String>) -> Self {
        Self { app, slugs }
    }
}

#[async_trait]
impl Task for LoadPlayersStatsTask {
    fn name(&self) -> String {
        "load players stats".to_string()
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
        let stats_service = resolve_trait!(StatsService);

        // Get stats
        match stats_service.get_stats(&self.slugs).await {
            Ok(stats) => {
                // Update player in app state
                let mut app = self.app.lock().await;
                app.state.merge_stats(stats);
                // After update: run strategies
                for s in &self.slugs {
                    app.run_strategies(&s.clone()).await;
                }
                
            }
            Err(err) => {
                log::error!(
                    "Failed to load stats for {}: {}",
                    &self.id(),
                    err.to_string()
                )
            }
        };
    }
}
