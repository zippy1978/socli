# TODO

- price history chart
- liquidity : mesure volume in time for each player, then rank them
- injured status

query GetPlayersStats($slugs: [String!]) {
  
  nbaPlayers(slugs: $slugs) {
    slug
    tenGameAverageGameStats {
      score
      detailedStats {
        secondsPlayed
      }
    }
    playerInjury {
      description
    }
    latestFinalGameStats (last: 10) {
      score 
      detailedStats {
        secondsPlayed
      }
      game {
        startDate
      }
    }
  }
}