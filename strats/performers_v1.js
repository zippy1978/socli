export function decide(player) {

    const lastPrice = parseFloat(player.prices[0].eur);
    
    if (player.stats.score > 35) {
      return {
        action: "Buy",
        comment: `score: ${player.stats.score}, price: ${lastPrice}€`,
      };
    }
  }
  