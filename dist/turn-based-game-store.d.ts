import { AgentPubKeyB64, Dictionary, EntryHashB64 } from '@holochain-open-dev/core-types';
import { ProfilesStore } from '@holochain-open-dev/profiles';
import { TurnBasedGameService } from './turn-based-game-service';
import { GameEntry, MoveInfo } from './types';
export interface GameState<M> {
    entry: GameEntry;
    moves: Array<MoveInfo<M>>;
}
export declare class TurnBasedGameStore<M> {
    #private;
    protected turnBasedGameService: TurnBasedGameService;
    profilesStore: ProfilesStore;
    game(gameHash: EntryHashB64): import("svelte/store").Readable<GameState<M>>;
    myGames: import("svelte/store").Readable<Dictionary<GameEntry>>;
    get myAgentPubKey(): string;
    constructor(turnBasedGameService: TurnBasedGameService, profilesStore: ProfilesStore);
    opponent(game: GameEntry): AgentPubKeyB64;
    /** Backend actions */
    fetchGame(gameHash: EntryHashB64): Promise<void>;
    fetchMyCurrentGames(): Promise<void>;
    makeMove(gameHash: EntryHashB64, move: M): Promise<void>;
    fetchGameMoves(gameHash: EntryHashB64): Promise<void>;
    private handleNewGameStarted;
    private handleNewMove;
    private decodeMove;
}
