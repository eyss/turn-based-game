import { AgentPubKeyB64 } from '@holochain-open-dev/core-types';
import { ProfilesStore } from '@holochain-open-dev/profiles';
import { HoloHashed } from '@holochain/conductor-api';
import { EloService } from './elo-service';
import { GameResult } from './types';
export declare enum ShortResult {
    Win = 1,
    Loss = 0,
    Draw = 0.5
}
export declare class EloStore {
    #private;
    protected eloService: EloService;
    profilesStore: ProfilesStore;
    elos: import("svelte/store").Readable<{
        [key: string]: number;
    }>;
    eloRanking: import("svelte/store").Readable<{
        agentPubKey: string;
        elo: number;
    }[]>;
    gameResults: import("svelte/store").Readable<{
        [key: string]: [HoloHashed<any>, GameResult][];
    }>;
    myElo: import("svelte/store").Readable<number>;
    myGameResults: import("svelte/store").Readable<[HoloHashed<any>, GameResult][]>;
    get myAgentPubKey(): string;
    constructor(eloService: EloService, profilesStore: ProfilesStore);
    /** Helpers for the types */
    getOpponent(gameResult: GameResult): AgentPubKeyB64;
    getMyResult(gameResult: GameResult): number;
    /** Backend actions */
    fetchMyGameResults(): Promise<void>;
    fetchMyElo(): Promise<void>;
    fetchGameResultsForAgents(agents: AgentPubKeyB64[]): Promise<void>;
    fetchEloForAgents(agents: AgentPubKeyB64[]): Promise<void>;
    private handleNewGameResult;
}
