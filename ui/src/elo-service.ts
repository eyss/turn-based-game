import { AgentPubKeyB64, EntryHashB64 } from '@holochain-open-dev/core-types';
import { CellClient } from '@holochain-open-dev/cell-client';
import { HoloHashed } from '@holochain/conductor-api';

import { GameResult } from './types';

export class EloService {
  constructor(public cellClient: CellClient, protected zomeName: string) {}

  public getGameResultsForAgents(agents: AgentPubKeyB64[]): Promise<{
    [key: string]: Array<[HoloHashed<any>, GameResult]>;
  }> {
    return this.callZome('get_game_results_for_agents', agents);
  }

  public getEloRatingForAgents(agents: AgentPubKeyB64[]): Promise<{
    [key: string]: number;
  }> {
    return this.callZome('get_elo_rating_for_agents', agents);
  }

  // TODO: remove when schedule lands
  public resolveFlags(): Promise<void> {
    return this.callZome(
      'scheduled_try_resolve_unpublished_game_results',
      null
    );
  }

  // TODO: remove when postcommit lands
  public linkGameResults(entryHashes: EntryHashB64[]): Promise<void> {
    return this.callZome('link_my_game_results', entryHashes);
  }

  private callZome(fnName: string, payload: any): Promise<any> {
    return this.cellClient.callZome(this.zomeName, fnName, payload);
  }
}
