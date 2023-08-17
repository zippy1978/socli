use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use quartermaster::task::Task;
use tokio::time::sleep;

use crate::app::{state::AppState, App};

pub struct RefreshPlayersDetailsTask {
    app: Arc<tokio::sync::Mutex<App>>,
}
impl RefreshPlayersDetailsTask {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
    }
}

#[async_trait]
impl Task for RefreshPlayersDetailsTask {
    fn name(&self) -> String {
        "refresh players details".to_string()
    }

    fn id(&self) -> String {
        "refresh players details".to_string()
    }

    async fn run(&self) {
        let mut index = 0;

        let mut players_slugs_for_refresh = vec![];

        loop {
            // Iterate on players every X secs and schedule a load prices task
            let mut app = self.app.lock().await;
            if let AppState::Initialized { players, .. } = &app.state {
                let player_count = players.len();
                if let Some(p) = players.get(index) {
                    let slug = p.slug.clone();
                    players_slugs_for_refresh.push(slug.clone());
                    app.refresh_player_prices(index, true).await;
                    if (index + 1) == player_count {
                        index = 0;
                    } else {
                        index += 1;
                    }
                    // Every 5, refreshes, player stats and injury are bulk refreshed
                    if players_slugs_for_refresh.len() == 5 {
                        
                        app.refresh_players_stats(&players_slugs_for_refresh).await;
                        app.refresh_players_injury(&players_slugs_for_refresh).await;

                        players_slugs_for_refresh.clear();
                    }

                }
              
            }
            std::mem::drop(app);

            // Wait
            sleep(Duration::from_secs(3)).await;
        }
    }
}
