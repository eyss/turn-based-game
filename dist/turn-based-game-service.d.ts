import { Dictionary, EntryHashB64, HeaderHashB64 } from '@holochain-open-dev/core-types';
import { CellClient } from '@holochain-open-dev/cell-client';
import { GameEntry, MoveInfo } from './types';
export declare class TurnBasedGameService {
    cellClient: CellClient;
    protected zomeName: string;
    constructor(cellClient: CellClient, zomeName: string);
    /** These functions **must** match the functions defined in lib/src/mixin.rs */
    getGameMoves(gameHash: EntryHashB64): Promise<Array<MoveInfo<any>>>;
    makeMove(gameHash: EntryHashB64, previousMoveHash: HeaderHashB64 | undefined, move: any): Promise<HeaderHashB64>;
    getMyCurrentGames(): Promise<Dictionary<GameEntry>>;
    getGame(gameHash: EntryHashB64): Promise<GameEntry>;
    private callZome;
}
