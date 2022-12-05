use std::sync::Arc;

use eyre::Result;
use log::{error, info};

use super::IoEvent;
use crate::{
    app::{
        task::{
            load_player_prices::LoadPlayerPricesTask, load_players::LoadPlayersTask,
            load_players_stats::LoadPlayersStatsTask,
            refresh_players_details::RefreshPlayersDetailsTask, run_strategies::RunStrategiesTask,
        },
        App,
    },
    core::{service::player::PlayerError, MainTaskManager},
    resolve,
};

#[derive(Debug)]
pub enum IoAsyncHandlerError {
    Player(String),
}

impl From<PlayerError> for IoAsyncHandlerError {
    fn from(e: PlayerError) -> Self {
        Self::Player(e.to_string())
    }
}

/// In the IO thread, we handle IO event without blocking the UI thread
pub struct IoAsyncHandler {
    app: Arc<tokio::sync::Mutex<App>>,
}

impl IoAsyncHandler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
    }

    /// We could be async here
    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Initialize => self.do_initialize().await,
            IoEvent::LoadPlayerDetails(slug) => self.do_load_player_prices(&slug).await,
            IoEvent::LoadPlayersStats(slugs) => self.do_load_players_stats(slugs).await,
            IoEvent::RunStrategies(slug) => self.do_run_strategies(&slug).await,
        };

        if let Err(err) = result {
            error!("Oops, something wrong happen: {:?}", err);
        }

        let mut app = self.app.lock().await;
        app.loaded();
    }

    async fn do_run_strategies(&mut self, slug: &str) -> Result<(), IoAsyncHandlerError> {
        let task_manager = resolve!(MainTaskManager);
        let app = self.app.lock().await;
        if let Some(player) = app.state.get_player(slug) {
            task_manager
                .run(Box::new(RunStrategiesTask::new(self.app.clone(), player.clone())))
                .await;
        }

        Ok(())
    }

    async fn do_load_players_stats(
        &mut self,
        slugs: Vec<String>,
    ) -> Result<(), IoAsyncHandlerError> {
        let task_manager = resolve!(MainTaskManager);
        task_manager
            .run(Box::new(LoadPlayersStatsTask::new(self.app.clone(), slugs)))
            .await;

        Ok(())
    }

    async fn do_load_player_prices(&mut self, slug: &str) -> Result<(), IoAsyncHandlerError> {
        let task_manager = resolve!(MainTaskManager);
        task_manager
            .run(Box::new(LoadPlayerPricesTask::new(self.app.clone(), slug)))
            .await;

        Ok(())
    }

    async fn do_initialize(&mut self) -> Result<(), IoAsyncHandlerError> {
        let task_manager = resolve!(MainTaskManager);
        info!("🚀 Initialize the application");
        let mut app = self.app.lock().await;

        app.initialize(vec![]).await; // we could update the app state
        info!("👍 Application initialized");

        // Load players
        task_manager
            .run(Box::new(LoadPlayersTask::new(self.app.clone())))
            .await;

        // Start background player details refresh
        task_manager
            .run(Box::new(RefreshPlayersDetailsTask::new(self.app.clone())))
            .await;

        Ok(())
    }

    /*async fn do_sleep(&mut self, duration: Duration) -> Result<()> {
        info!("😴 Go sleeping for {:?}...", duration);
        tokio::time::sleep(duration).await;
        info!("⏰ Wake up !");
        // Notify the app for having slept
        let mut app = self.app.lock().await;
        app.sleeped();

        Ok(())
    }*/
}
