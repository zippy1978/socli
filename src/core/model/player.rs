use chrono::{DateTime, Utc};
use rquickjs::IntoJs;
use serde::{Deserialize, Serialize};

use super::{currency::Currency, price::Price, stats::Stats, injury::Injury};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoJs)]
pub struct Player {
    pub slug: String,
    pub display_name: String,
    pub birth_date: String,
    pub team: Option<String>,
    pub prices: Vec<Price>,
    pub stats: Option<Stats>,
    pub injury: Option<Injury>,
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

    pub fn price_avg(&self, currency: Currency, max_count: usize) -> Option<f64> {
        if self.prices.is_empty() {
            return None;
        }

        // Filter prices according to max_count
        let prices = self.prices.iter().take(max_count);
        let len = prices.len() as f64;

        // Compute average
        Some(match currency {
            Currency::Euro => prices.map(|p| p.eur.parse::<f64>().unwrap()).sum::<f64>() / len,
            Currency::Usd => prices.map(|p| p.usd.parse::<f64>().unwrap()).sum::<f64>() / len,
        })
    }

    pub fn age(&self) -> u32 {
        let birth_date = DateTime::parse_from_rfc3339(&self.birth_date).unwrap().with_timezone(&Utc);
        let now = chrono::Utc::now().date_naive();
        match now.years_since(birth_date.date_naive()) {
            Some(a) => a,
            None => 0,
        }
    }
}
