use chrono::{DateTime, FixedOffset, Utc};
use rquickjs::IntoJs;
use serde::{Deserialize, Serialize};

use super::{currency::Currency, injury::Injury, price::Price, stats::Stats};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoJs)]
pub struct Player {
    pub slug: String,
    pub display_name: String,
    pub birth_date: String,
    pub team: Option<String>,
    pub prices: Vec<Price>,
    pub stats: Option<Stats>,
    pub injury: Option<Injury>,
    pub positions: Vec<String>,
    pub country: String,
    pub number: i64,
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
        let birth_date = DateTime::parse_from_rfc3339(&self.birth_date)
            .unwrap()
            .with_timezone(&Utc);
        let now = chrono::Utc::now().date_naive();
        match now.years_since(birth_date.date_naive()) {
            Some(a) => a,
            None => 0,
        }
    }

    pub fn rank(&self, players: &[Player]) -> Option<usize> {
        let all_scores_loaded = players.iter().find(|p| p.stats.is_none()).is_none();
        if self.stats.is_some() {
            if all_scores_loaded {
                let mut sorted_players = players.to_vec();
                sorted_players.sort_by(|a, b| {
                    b.stats
                        .as_ref()
                        .unwrap()
                        .score
                        .cmp(&a.stats.as_ref().unwrap().score)
                });
                let player_rank = sorted_players
                    .iter()
                    .position(|p| p.slug == self.slug)
                    .unwrap()
                    + 1;
                return Some(player_rank);
            }
        }
        None
    }

    pub fn sales_hours_interval_avg(&self) -> Option<f64> {
        if !self.prices.is_empty() {
            let sales_dates = self
                .prices
                .iter()
                .rev()
                .map(|p| DateTime::parse_from_rfc3339(&p.date).unwrap())
                .collect::<Vec<DateTime<FixedOffset>>>();

            let mut intervals = Vec::new();

            for i in 0..sales_dates.len() - 1 {
                let interval = sales_dates[i + 1].signed_duration_since(sales_dates[i]);
                intervals.push(interval);
            }

            let total_duration: i64 = intervals.iter().map(|d| d.num_hours()).sum();
            let average_duration = total_duration as f64 / intervals.len() as f64;
            return Some(average_duration);
        }
        None
    }
}
