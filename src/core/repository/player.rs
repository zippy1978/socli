use std::collections::HashSet;

use async_trait::async_trait;

use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use regex::Regex;
use reqwest::Client;

use crate::core::model::player::Player;

use super::error::RepoError;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/core/graphql/schema-us-sports.graphql",
    query_path = "src/core/graphql/get-players-info-query.graphql",
    response_derives = "Debug"
)]
struct GetPlayersInfo;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/core/graphql/schema.graphql",
    query_path = "src/core/graphql/get-all-tokens-nba-query.graphql",
    response_derives = "Debug"
)]
struct GetAllTokensNBA;

#[async_trait]
pub trait PlayerRepo {
    async fn get_players(&self, limit: usize) -> Result<Vec<Player>, RepoError>;
}

pub struct PlayerRepoImpl {
    client: Client,
}

impl PlayerRepoImpl {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("graphql-rust/0.10.0")
                .build()
                .expect("failed to build HTTP client"),
        }
    }

    pub(crate) fn parse_player_slug(&self, token_slug: &str) -> Result<String, RepoError> {
        let re = Regex::new(r"([a-z].*)(-[0-9].*-){1}").unwrap();
        let caps = re.captures(token_slug).unwrap();
        let result = caps.get(1).map_or("", |m| m.as_str()).to_string();

        Ok(result)
    }

    async fn get_players_info(&self, player_slugs: &[String]) -> Result<Vec<Player>, RepoError> {
        // Run GraphQL query to retrieve players info
        let variables = get_players_info::Variables {
            slugs: Some(player_slugs.to_vec()),
        };
        let response_body = post_graphql::<GetPlayersInfo, _>(
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
        let nba_players = response_data.nba_players;

        let mut players = vec![];
        for p in nba_players {
            players.push(Player {
                slug: p.slug.clone(),
                display_name: p.display_name,
                team: match p.team {
                    Some(t) => Some(t.name),
                    None => None,
                },
                prices: vec![],
                stats: None,
            });
        }
        Ok(players)
    }

    async fn get_players_page(
        &self,
        size: Option<i64>,
        cursor: Option<String>,
    ) -> Result<(Vec<Player>, Option<String>), RepoError> {
        // Run GraphQL query to retrieve all tokens
        let variables = get_all_tokens_nba::Variables {
            cursor: cursor,
            size: size,
        };

        let response_body = post_graphql::<GetAllTokensNBA, _>(
            &self.client,
            "https://api.sorare.com/graphql",
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
        let nodes = response_data.tokens.all_nfts.nodes;

        let mut player_slugs = vec![];
        for n in nodes {
            player_slugs.push(self.parse_player_slug(&n.slug)?);
        }

        let next = response_data.tokens.all_nfts.page_info.end_cursor;

        Ok((self.get_players_info(&player_slugs).await?, next))
    }
}

#[async_trait]
impl PlayerRepo for PlayerRepoImpl {
    async fn get_players(&self, limit: usize) -> Result<Vec<Player>, RepoError> {
        
        // Query all pages
        let mut players_set = HashSet::new();
        log::debug!("Start player loading");
        let mut result = self.get_players_page(Some(50), None).await?;
        players_set.extend(result.0);
        log::debug!("Loaded players count is: {}", players_set.len());
        while result.1.is_some() && players_set.len() < limit {
            let cursor = result.1.clone().unwrap();
            log::debug!("Loading player page at cursor {}", &cursor);
            result = self.get_players_page(Some(50), result.1.clone()).await?;
            players_set.extend(result.0);
            log::debug!("Loaded players count is: {}", players_set.len());
        }

        // Convert to vec and sort results
        let mut players = players_set.into_iter().collect::<Vec<Player>>();
        players.sort_by(|a, b| a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()));


        Ok(players)
    }
}
