import { AgentPubKeyB64, EntryHashB64 } from '@holochain-open-dev/core-types';
import { CellClient } from '@holochain-open-dev/cell-client';
import { HoloHashed } from '@holochain/conductor-api';
import { GameResult } from './types';
export declare class EloService {
    cellClient: CellClient;
    protected zomeName: string;
    constructor(cellClient: CellClient, zomeName: string);
    getGameResultsForAgents(agents: AgentPubKeyB64[]): Promise<{
        [key: string]: Array<[HoloHashed<any>, GameResult]>;
    }>;
    getEloRatingForAgents(agents: AgentPubKeyB64[]): Promise<{
        [key: string]: number;
    }>;
    resolveFlags(): Promise<void>;
    linkGameResults(entryHashes: EntryHashB64[]): Promise<void>;
    private callZome;
}
