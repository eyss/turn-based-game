import {
  Orchestrator,
  Config,
  InstallAgentsHapps,
  Player,
} from "@holochain/tryorama";
import path from "path";

const conductorConfig = Config.gen();

// Construct proper paths for your DNAs
const tictactoe = path.join(
  __dirname,
  "../../example/workdir/tictactoe-test.dna"
);

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [tictactoe],
  ],
  [
    // happ 0
    [tictactoe],
  ],
];

const createGame = (caller) => (rival) =>
  caller.call("tictactoe", "create_tictactoe_game", rival);
const getMyCurrentGames = (caller) => () =>
  caller.call("tictactoe", "get_my_current_games", null);

const createMove = (caller) => (gameHash, previousMoveHash, x, y) =>
  caller.call("tictactoe", "make_move", {
    game_hash: gameHash,
    previous_move_hash: previousMoveHash,
    game_move: {
      Place: {
        x,
        y,
      },
    },
  });

const getWinner = (caller) => (gameHash) =>
  caller.call("tictactoe", "get_winner", gameHash);

const getState = (caller) => (gameHash) =>
  caller.call("tictactoe", "get_game_state", gameHash);

const sleep = (ms) =>
  new Promise((resolve) => setTimeout(() => resolve(null), ms));

const orchestrator = new Orchestrator();

orchestrator.registerScenario("add and retrieve a book", async (s, t) => {
  const [player]: Player[] = await s.players([conductorConfig]);

  // install your happs into the coductors and destructuring the returned happ data using the same
  // array structure as you created in your installation array.
  const [[alice_common], [bob_common]] = await player.installAgentsHapps(
    installation
  );

  const alice = alice_common.cells[0];
  const bob = bob_common.cells[0];

  const aliceAddress = await alice.call("tictactoe", "who_am_i", null);
  const bobAddress = await bob.call("tictactoe", "who_am_i", null);

  let result;
  let lastMoveHash;
  try {
    result = await createGame(alice)(aliceAddress);
    t.ok(false);
  } catch (e) {
    t.ok(true);
  }

  result = await createGame(alice)(bobAddress);
  t.ok(result);
  await sleep(4000);

  const currentGames = await getMyCurrentGames(alice)();
  t.equal(Object.keys(currentGames).length, 1);

  let gameAddress = result;

  result = await getWinner(alice)(gameAddress);
  t.deepEqual(result, null);

  result = await getState(alice)(gameAddress);
  t.deepEqual(result, {
    player_1: [],
    player_2: [],
    player_resigned: null,
  });

  try {
    result = await createMove(alice)(gameAddress, null, 0, 0);
    t.ok(false);
  } catch (e) {
    t.ok(true);
  }

  try {
    result = await createMove(bob)(gameAddress, null, 4, 0);
    t.ok(false);
  } catch (e) {
    t.ok(true);
  }

  lastMoveHash = await createMove(bob)(gameAddress, null, 0, 0);
  t.ok(lastMoveHash);
  await sleep(4000);

  try {
    result = await createMove(alice)(gameAddress, lastMoveHash, 0, 0);
    t.ok(false);
  } catch (e) {
    t.ok(true);
  }

  lastMoveHash = await createMove(alice)(gameAddress, lastMoveHash, 1, 0);
  t.ok(lastMoveHash);
  await sleep(4000);

  lastMoveHash = await createMove(bob)(gameAddress, lastMoveHash, 0, 1);
  t.ok(lastMoveHash); // Fail
  await sleep(4000);

  lastMoveHash = await createMove(alice)(gameAddress, lastMoveHash, 1, 1);
  t.ok(lastMoveHash);
  await sleep(4000);

  lastMoveHash = await createMove(bob)(gameAddress, lastMoveHash, 0, 2);
  t.ok(lastMoveHash);
  await sleep(4000);

  result = await getWinner(alice)(gameAddress);
  t.deepEqual(result, 0);

  result = await getState(alice)(gameAddress);
  t.deepEqual(result, {
    player_1: [
      { x: 0, y: 0 },
      { x: 0, y: 1 },
      { x: 0, y: 2 },
    ],
    player_2: [
      { x: 1, y: 0 },
      { x: 1, y: 1 },
    ],
    player_resigned: null,
  });

  try {
    result = await createMove(alice)(gameAddress, lastMoveHash, 2, 2);
    t.ok(false);
  } catch (e) {
    t.ok(true);
  }
});

orchestrator.run();
