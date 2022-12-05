use std::sync::Arc;

use async_trait::async_trait;
use quartermaster::task::Task;

use crate::{app::App, core::service::price::PriceService, resolve_trait};

pub struct LoadPlayerPricesTask {
    app: Arc<tokio::sync::Mutex<App>>,
    slug: String,
}
impl LoadPlayerPricesTask {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>, slug: &str) -> Self {
        Self {
            app,
            slug: slug.to_string(),
        }
    }
}

#[async_trait]
impl Task for LoadPlayerPricesTask {
    fn name(&self) -> String {
        "load player details".to_string()
    }

    fn id(&self) -> String {
        self.slug.to_string()
    }

    async fn run(&self) {
        let price_service = resolve_trait!(PriceService);

        // Get prices
        match price_service.get_prices(&self.slug).await {
            Ok(prices) => {
                // Update player in app state
                let mut app = self.app.lock().await;
                app.state.merge_prices(&self.slug,prices);
                 // After update: run strategies
                 app.run_strategies(&self.slug.clone()).await;
            }
            Err(err) => {
                log::error!(
                    "Failed to load prices for {}: {}",
                    &self.slug,
                    err.to_string()
                )
            }
        };
    }
}
