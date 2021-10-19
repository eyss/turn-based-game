import {
  AgentPubKeyB64,
  EntryHashB64,
  serializeHash,
} from '@holochain-open-dev/core-types';
import { ProfilesStore } from '@holochain-open-dev/profiles';
import { HoloHashed } from '@holochain/conductor-api';
import { derived, writable, Writable } from 'svelte/store';
import { EloService } from './elo-service';
import { GameResult } from './types';
import { headerTimestamp } from './utils';

export enum ShortResult {
  Win = 1.0,
  Loss = 0.0,
  Draw = 0.5,
}

export class EloStore {
  #gameResultsByAgent: Writable<{
    [key: string]: Array<[HoloHashed<any>, GameResult]>;
  }> = writable({});

  #elosByAgent: Writable<{
    [key: string]: number;
  }> = writable({});

  public elos = derived(this.#elosByAgent, i => i);

  public eloRanking = derived(this.#elosByAgent, i =>
    Object.entries(i)
      .map(([agentPubKey, elo]) => ({ agentPubKey, elo }))
      .sort((a, b) => b.elo - a.elo)
  );

  public gameResults = derived(this.#gameResultsByAgent, i => i);

  public myElo = derived(this.#elosByAgent, i => i[this.myAgentPubKey]);

  public myGameResults = derived(this.#gameResultsByAgent, i => {
    const myResults = i[this.myAgentPubKey];
    if (!myResults) return [];
    return myResults.sort(
      (
        gr1,
        gr2 // TODO: fix this
      ) => headerTimestamp(gr2[0]) - headerTimestamp(gr1[0])
    );
  });

  public get myAgentPubKey() {
    return serializeHash(this.eloService.cellClient.cellId[1]);
  }

  constructor(
    protected eloService: EloService,
    public profilesStore: ProfilesStore
  ) {
    // TODO: remove when scheduler actually works
    setInterval(() => this.eloService.resolveFlags(), 5000);
    this.eloService.resolveFlags();

    this.eloService.cellClient.addSignalHandler(signal => {
      if (signal.data.payload.type === 'NewGameResult') {
        this.handleNewGameResult(
          signal.data.payload.game_result,
          signal.data.payload.entry_hash,
          signal.data.payload.are_links_missing
        );
      }
    });
  }

  /** Helpers for the types */

  getOpponent(gameResult: GameResult): AgentPubKeyB64 {
    if (gameResult.player_a.player_address === this.myAgentPubKey)
      return gameResult.player_b.player_address;
    return gameResult.player_a.player_address;
  }

  getMyResult(gameResult: GameResult): number {
    if (gameResult.player_a.player_address) return gameResult.score_player_a;
    return 1 - gameResult.score_player_a;
  }

  /** Backend actions */

  async fetchMyGameResults() {
    return this.fetchGameResultsForAgents([this.myAgentPubKey]);
  }

  async fetchMyElo() {
    return this.fetchEloForAgents([this.myAgentPubKey]);
  }

  async fetchGameResultsForAgents(agents: AgentPubKeyB64[]): Promise<void> {
    const gameResults = await this.eloService.getGameResultsForAgents(agents);

    this.#gameResultsByAgent.update(r => ({ ...r, ...gameResults }));
  }

  async fetchEloForAgents(agents: AgentPubKeyB64[]): Promise<void> {
    const elos = await this.eloService.getEloRatingForAgents(agents);
    this.#elosByAgent.update(e => ({ ...e, ...elos }));
  }

  private async handleNewGameResult(
    gameResult: GameResult,
    gameResultHash: EntryHashB64,
    areLinksMissing: boolean
  ) {
    // TODO: remove when post_commit lands

    if (areLinksMissing) {
      await this.eloService.linkGameResults([gameResultHash]);
    }

    const players = [
      gameResult.player_a.player_address,
      gameResult.player_b.player_address,
    ];

    const promises = [
      this.fetchGameResultsForAgents(players),
      this.fetchEloForAgents(players),
      this.profilesStore.fetchAgentsProfiles(players),
    ];
    await Promise.all(promises);
  }
}
