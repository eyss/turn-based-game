const createGame = (caller) => (rival) =>
  caller.call("tictactoe", "tictactoe", "create_game", {
    rival,
    timestamp: Math.floor(Date.now() / 1000),
  });

const createMove = (caller) => (gameAddress, x, y) =>
  caller.call("tictactoe", "tictactoe", "place_piece", {
    game_address: gameAddress,
    x,
    y,
  });

const getWinner = (caller) => (gameAddress, x, y) =>
  caller.call("tictactoe", "tictactoe", "get_winner", {
    game_address: gameAddress,
  });

const getState = (caller) => (gameAddress) =>
  caller.call("tictactoe", "tictactoe", "get_game_state", {
    game_address: gameAddress,
  });

const getAgentGames = (caller) => (agentAddress) =>
  caller.call("tictactoe", "tictactoe", "get_agent_games", {
    agent_address: agentAddress,
  });

module.exports = {
  getAgentGames,
  createGame,
  createMove,
  getWinner,
  getState,
};
