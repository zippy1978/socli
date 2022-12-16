export function decide(player) {
  const today = new Date();
  const birthDate = new Date(player.birth_date);
  const age = today.getFullYear() - birthDate.getFullYear();
  if (age < 25) {
    return {
      action: "Buy",
      comment: `age: ${age}, score: ${player.stats.score}`,
    };
  }
}
