use async_trait::async_trait;
use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use reqwest::Client;

use crate::core::model::price::Price;

use super::error::RepoError;

type ISO8601DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/core/graphql/schema.graphql",
    query_path = "src/core/graphql/get-prices-query.graphql",
    response_derives = "Debug"
)]
struct GetPrices;

#[async_trait]
pub trait PriceRepo {
    // TODO: support rarity
    async fn get_prices(&self, player_slug: &str) -> Result<Vec<Price>, RepoError>;
}

pub struct PriceRepoImpl {
    client: Client,
}

impl PriceRepoImpl {
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
impl PriceRepo for PriceRepoImpl {
    async fn get_prices(&self, player_slug: &str) -> Result<Vec<Price>, RepoError> {
        // Run GraphQL query to retrieve player prices
        let variables = get_prices::Variables {
            slug: player_slug.to_string(),
        };
        let response_body =
            post_graphql::<GetPrices, _>(&self.client, "https://api.sorare.com/graphql", variables)
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
        let token_prices = response_data.tokens.token_prices;
        Ok(token_prices
            .iter()
            .map(|tp| Price {
                player_slug: player_slug.to_string(),
                date: tp.date.clone(),
                eur: format!("{:.2}", tp.amount_in_fiat.eur),
                usd: format!("{:.2}", tp.amount_in_fiat.usd),
            })
            .collect())
    }
}
