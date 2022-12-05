const MAX_POINT_PRICE = 0.4;
const MIN_PROGRESSION_RATIO = 0;

export function decide(player) {
  const gamesPlayed = player.stats.games.filter((g) => g.did_play);
  const gamesPlayedCount = gamesPlayed.length;
  const gamesCount = player.stats.games.length;
  const gamePlayedRatio = gamesPlayedCount / gamesCount;
  const didPlayLastGame = player.stats.games.length > 0 ? player.stats.games[0].did_play : false;
  const lastPrice = parseFloat(player.prices[0].eur);
  const pointPrice = lastPrice / player.stats.score + (lastPrice / player.stats.score) * (1 - gamePlayedRatio);
  const scoreProgressionRatio = gamesPlayedCount > 0 ? gamesPlayed[0].score / player.stats.score - 1 : 0;

  if (didPlayLastGame && pointPrice < MAX_POINT_PRICE && scoreProgressionRatio >= MIN_PROGRESSION_RATIO && player.stats.score > 0) {
    return {
      action: "Buy",
      player_slug: player.slug,
      player_name: player.display_name,
      strategy: "underdog_v1",
      comment: `price: ${lastPrice.toFixed(2)}€, point price: ${pointPrice.toFixed(2)}€, score progression: ${(scoreProgressionRatio * 100).toFixed(2)}%`,
    };
  }
}
