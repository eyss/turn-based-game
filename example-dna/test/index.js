/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example

const path = require("path");

const {
  Orchestrator,
  Config,
  combine,
  singleConductor,
  localOnly,
  tapeExecutor,
} = require("@holochain/tryorama");

process.on("unhandledRejection", (error) => {
  // Will print "unhandledRejection err is not defined"
  console.error("got unhandledRejection:", error);
});

const dnaPath = path.join(__dirname, "../dist/example-dna.dna.json");
const dna = Config.dna(dnaPath, "scaffold-test");
const conductorConfig = Config.gen(
  { tictactoe: dna },
  {
    network: {
      type: "sim2h",
      sim2h_url: "ws://localhost:9000",
    },
  }
);

const orchestrator = new Orchestrator({
  waiter: {
    softTimeout: 20000,
    hardTimeout: 30000,
  },
});

const { createGame, createMove, getAgentGames, getWinner, getState } = require("./utils");

orchestrator.registerScenario(
  "play a tictactoe game succeeds",
  async (s, t) => {
    const { alice, bob } = await s.players(
      { alice: conductorConfig, bob: conductorConfig },
      true
    );
    const aliceAddress = alice.instance("tictactoe").agentAddress;
    const bobAddress = bob.instance("tictactoe").agentAddress;

    let result = await createGame(alice)(aliceAddress);
    t.notOk(result.Ok);
    await s.consistency();

    result = await createGame(alice)(bobAddress);
    t.ok(result.Ok);
    await s.consistency();

    let gameAddress = result.Ok;

    result = await getAgentGames(bob)(aliceAddress);
    t.equal(result.Ok.length, 1);

    result = await getAgentGames(alice)(bobAddress);
    t.equal(result.Ok.length, 1);

    result = await getWinner(alice)(gameAddress);
    t.equal(result.Ok, null);

    result = await getState(alice)(gameAddress);
    t.deepEqual(result.Ok, { player_1: [], player_2: [] });

    result = await createMove(alice)(gameAddress, 0, 0);
    t.notOk(result.Ok);

    result = await createMove(bob)(gameAddress, 4, 0);
    t.notOk(result.Ok);

    result = await createMove(bob)(gameAddress, 0, 0);
    t.ok(result.Ok);
    await s.consistency();

    result = await createMove(alice)(gameAddress, 0, 0);
    t.notOk(result.Ok);

    result = await createMove(alice)(gameAddress, 1, 0);
    t.ok(result.Ok);
    await s.consistency();

    result = await createMove(bob)(gameAddress, 0, 1);
    t.ok(result.Ok);
    await s.consistency();

    result = await createMove(alice)(gameAddress, 1, 1);
    t.ok(result.Ok);
    await s.consistency();

    result = await createMove(bob)(gameAddress, 0, 2);
    t.ok(result.Ok);
    await s.consistency();

    result = await getWinner(alice)(gameAddress);
    t.equal(result.Ok, bobAddress);

    result = await getState(alice)(gameAddress);
    t.deepEqual(result.Ok, { player_1: [ { x: 0, y: 0 }, { x: 0, y: 1 }, { x: 0, y: 2 } ], player_2: [ { x: 1, y: 0 }, { x: 1, y: 1 } ] });

    result = await createMove(alice)(gameAddress, 2, 2);
    t.notOk(result.Ok);
  }
);

orchestrator.run();
