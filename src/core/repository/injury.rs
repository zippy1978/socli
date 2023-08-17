use async_trait::async_trait;
use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use reqwest::Client;

use crate::core::model::injury::Injury;

use super::error::RepoError;

type Time = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/core/graphql/schema.graphql",
    query_path = "src/core/graphql/get-players-injury-query.graphql",
    response_derives = "Debug"
)]
struct GetPlayersInjury;

#[async_trait]
pub trait InjuryRepo {
    async fn get_injuries(&self, player_slugs: &[String]) -> Result<Vec<Injury>, RepoError>;
}

pub struct InjuryRepoImpl {
    client: Client,
}

impl InjuryRepoImpl {
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
impl InjuryRepo for InjuryRepoImpl {
    async fn get_injuries(&self, player_slugs: &[String]) -> Result<Vec<Injury>, RepoError> {
        // Run GraphQL query to retrieve players injury
        let variables = get_players_injury::Variables {
            slugs: Some(player_slugs.to_vec()),
        };

        let response_body = post_graphql::<GetPlayersInjury, _>(
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
            .filter(|p| p.player_injury.is_some())
            .map(|p| Injury {
                player_slug: p.slug.clone(),
                // Append time to date to keep homogeneity in date formats
                date: format!("{}{}", p.player_injury.as_ref().unwrap().start_date, "T00:00:00Z"),
                comment: p.player_injury.as_ref().unwrap().comment.clone(),
                description: p.player_injury.as_ref().unwrap().description.clone(),
                update_date: p.player_injury.as_ref().unwrap().update_date_time.clone(),
            })
            .collect())
    }
}
