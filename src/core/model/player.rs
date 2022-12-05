use rquickjs::IntoJs;
use serde::{Deserialize, Serialize};

use super::{currency::Currency, price::Price, stats::Stats};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoJs)]
pub struct Player {
    pub slug: String,
    pub display_name: String,
    pub team: Option<String>,
    pub prices: Vec<Price>,
    pub stats: Option<Stats>,
}

impl Player {
    pub fn price_delta_ratio(&self, currency: Currency) -> Option<f64> {
        if self.prices.is_empty() {
            return None;
        }

        let (last, old) = match currency {
            Currency::Euro => {
                let last_eur = self.prices.first().unwrap().eur.parse::<f64>().unwrap();
                let old_eur = self.prices.last().unwrap().eur.parse::<f64>().unwrap();
                (last_eur, old_eur)
            }
            Currency::Usd => {
                let last_usd = self.prices.first().unwrap().usd.parse::<f64>().unwrap();
                let old_usd = self.prices.last().unwrap().usd.parse::<f64>().unwrap();
                (last_usd, old_usd)
            }
        };

        Some((last - old) / old)
    }
}
