const createGame = (caller) => (rival) =>
  caller.call("tictactoe", "tictactoe", "create_game", {
    entry: { rival, timestamp: Date.now() / 1000 },
  });

const createMove = (caller) => (gameAddress, x, y) =>
  caller.call("tictactoe", "tictactoe", "create_game", {
    entry: { game_address: gameAddress, x, y },
  });

module.exports = {
  createGame,
  createMove,
};
