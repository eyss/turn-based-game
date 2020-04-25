const createGame = (caller) => (rival) =>
  caller.call("tictactoe", "tictactoe", "create_game", {
    rival,
    timestamp: Date.now() / 1000,
  });

const createMove = (caller) => (gameAddress, x, y) =>
  caller.call("tictactoe", "tictactoe", "place_piece", {
    game_address: gameAddress,
    x,
    y,
  });

module.exports = {
  createGame,
  createMove,
};
