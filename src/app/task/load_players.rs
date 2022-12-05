use std::sync::Arc;

use async_trait::async_trait;
use quartermaster::task::Task;

use crate::{app::App, core::service::player::PlayerService, resolve_trait};

pub struct LoadPlayersTask {
    app: Arc<tokio::sync::Mutex<App>>,
}
impl LoadPlayersTask {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
    }
}

#[async_trait]
impl Task for LoadPlayersTask {
    fn name(&self) -> String {
        "load players".to_string()
    }

    fn id(&self) -> String {
        "load players".to_string()
    }

    async fn run(&self) {
        let player_service = resolve_trait!(PlayerService);

        match player_service.get_players().await {
            Ok(players) => {
                let mut app = self.app.lock().await;
                app.initialize(players).await;
            }
            Err(e) => {
                log::error!("Failed to load players: {}", e.to_string())
            }
        }
    }
}
