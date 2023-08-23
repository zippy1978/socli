use std::sync::Arc;

use async_trait::async_trait;
use quartermaster::task::Task;

use crate::{
    app::App,
    core::service::player::{PlayerError, PlayerService},
    resolve_trait,
};

pub struct LoadPlayersTask {
    app: Arc<tokio::sync::Mutex<App>>,
}
impl LoadPlayersTask {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
    }
}

impl LoadPlayersTask {
    async fn load_players(&self) -> Result<(), PlayerError> {
        let player_service = resolve_trait!(PlayerService);

        match player_service.get_players().await {
            Ok(players) => {
                let mut app = self.app.lock().await;
                app.initialize(players).await;
                Ok(())
            }
            Err(e) => {
                // If failed to load : retry after storage clear
                log::error!("Failed to load players: {}", e.to_string());
                Err(e)
            }
        }
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

        let mut result = self.load_players().await;
        while !result.is_ok() {
            // Retry to laod player after storage clear
            log::info!("Retrying to load players");
            if player_service.clear_storage().await.is_ok() {
                result = self.load_players().await;
            } else {
                log::error!("Failed to clear storage");
            }
        }
    }
}
