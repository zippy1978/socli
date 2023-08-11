use async_trait::async_trait;
use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use reqwest::Client;

use crate::core::model::stats::{Game, Stats};

use super::error::RepoError;

type Time = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/core/graphql/schema.graphql",
    query_path = "src/core/graphql/get-players-stats-query.graphql",
    response_derives = "Debug"
)]
struct GetPlayersStats;

#[async_trait]
pub trait StatsRepo {
    async fn get_stats(&self, player_slugs: &[String]) -> Result<Vec<Stats>, RepoError>;
}

pub struct StatsRepoImpl {
    client: Client,
}

impl StatsRepoImpl {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("graphql-rust/0.10.0")
                .build()
                .expect("failed to build HTTP client"),
        }
    }
}

#[async_trait]
impl StatsRepo for StatsRepoImpl {
    async fn get_stats(&self, player_slugs: &[String]) -> Result<Vec<Stats>, RepoError> {
        // Run GraphQL query to retrieve players stats
        let variables = get_players_stats::Variables {
            slugs: Some(player_slugs.to_vec()),
        };

        let response_body = post_graphql::<GetPlayersStats, _>(
            &self.client,
            "https://api.sorare.com/sports/graphql",
            variables,
        )
        .await?;

        // Check errors
        if let Some(errors) = response_body.errors {
            if let Some(first_err) = errors.get(0) {
                return Err(RepoError::Read(first_err.to_string()));
            }
        }

        // Parse result
        if response_body.data.is_none() {
            return Err(RepoError::Read("no data".to_string()));
        }

        let response_data = response_body.data.unwrap();
        let players = response_data.nba_players;
        Ok(players
            .iter()
            .map(|p| Stats {
                player_slug: p.slug.clone(),
                score: p.ten_game_average_game_stats.score.round() as i64,
                games: p
                    .latest_final_game_stats
                    .iter()
                    .map(|gs| Game {
                        date: gs.game.start_date.clone(),
                        // Deprecated: maybe not usefull anymore
                        did_play: match &gs.detailed_stats {
                            Some(ds) => ds.seconds_played > 0,
                            None => false,
                        },
                        minutes_played: match &gs.detailed_stats {
                            Some(ds) => ds.seconds_played / 60,
                            None => 0,
                        },
                        score: gs.score.round() as u64,
                    })
                    .collect(),
            })
            .collect())
    }
}
