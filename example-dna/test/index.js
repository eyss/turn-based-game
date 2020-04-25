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

const orchestrator = new Orchestrator({
  middleware: combine(
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require("tape")),

    // specify that all "players" in the test are on the local machine, rather than
    // on remote machines
    localOnly,

    // squash all instances from all conductors down into a single conductor,
    // for in-memory testing purposes.
    // Remove this middleware for other "real" network types which can actually
    // send messages across conductors
    singleConductor
  ),
});

const dna = {
  id: "rolesTests",
  file: "./dist/example-dna.dna.json",
};
const aliceConfig = Config.gen({
  agent: {
    id: "alice",
    public_address:
      "HcScJWFagz6JtswwimIuXHa5V8h8Sjoy9Bkrbzfervjhuvq8g9whUEawSk845iz",
    file: "./alice.keystore",
  },
  dna,
});
const bobConfig = Config.gen({
  agent: {
    id: "bob",
    public_address:
      "HcSCJR7dD9t6Nuqc9kxqWfKzbo443p8y8Z38IA9dukBEr96umt3b47uetMXg3aa",
    file: "./bob.keystore",
  },
  dna,
});

const {
  assignRole,
  getAgentRoles,
  getAllRoles,
  getAgentsWithRole,
  createEntry,
  unassignRole,
} = require("./utils");

orchestrator.registerScenario(
  "only progenitor can assign roles",
  async (s, t) => {
    const { alice, bob } = await s.players(
      { alice: aliceConfig, bob: bobConfig },
      true
    );
    const aliceAddress = alice.instance("rolesTest").agentAddress;
    const bobAddress = bob.instance("rolesTest").agentAddress;

    let result = await assignRole(bob)(aliceAddress, "editor");
    t.notOk(result.Ok);

    result = await assignRole(alice)(bobAddress, "editor");
    t.ok(result.Ok);
    await s.consistency();

    result = await getAgentRoles(bob)(bobAddress);
    t.equal(result.Ok[0], "editor");

    result = await getAgentsWithRole(bob)("editor");
    t.equal(result.Ok[0], bobAddress);

    result = await getAllRoles(bob)();
    t.equal(result.Ok[0], "editor");
  }
);

orchestrator.registerScenario(
  "agents can only create entries when given permission",
  async (s, t) => {
    const { alice, bob } = await s.players(
      { alice: aliceConfig, bob: bobConfig },
      true
    );
    const aliceAddress = alice.instance("rolesTest").agentAddress;
    const bobAddress = alice.instance("rolesTest").agentAddress;

    let result = await createEntry(alice)();
    t.notOk(result.Ok);

    result = await createEntry(bob)();
    t.notOk(result.Ok);

    result = await assignRole(alice)(bobAddress, "editor");
    t.ok(result.Ok);
    await s.consistency();

    result = await createEntry(bob)();
    t.ok(result.Ok);

    result = await unassignRole(alice)(bobAddress, "editor");
    t.ok(result.Ok);
    await s.consistency();

    result = await createEntry(bob)();
    t.notOk(result.Ok);
  }
);

orchestrator.run();
