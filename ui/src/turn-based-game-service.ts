import {
  AgentPubKeyB64,
  Dictionary,
  EntryHashB64,
  HeaderHashB64,
} from '@holochain-open-dev/core-types';
import { CellClient } from '@holochain-open-dev/cell-client';

import { GameEntry, GameMoveEntry, MoveInfo } from './types';

export class TurnBasedGameService {
  constructor(public cellClient: CellClient, protected zomeName: string) {}

  /** These functions **must** match the functions defined in lib/src/mixin.rs */

  public getGameMoves(gameHash: EntryHashB64): Promise<Array<MoveInfo<any>>> {
    return this.callZome('get_game_moves', gameHash);
  }

  public getMyCurrentGames(): Promise<Dictionary<GameEntry>> {
    return this.callZome('get_my_current_games', null);
  }

  private callZome(fnName: string, payload: any): Promise<any> {
    return this.cellClient.callZome(this.zomeName, fnName, payload);
  }
}