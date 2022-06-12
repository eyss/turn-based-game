var _TurnBasedGameStore_gamesByEntryHash;
import { __classPrivateFieldGet } from "tslib";
import { serializeHash, } from '@holochain-open-dev/core-types';
import { decode } from '@msgpack/msgpack';
import { derived, get, writable } from 'svelte/store';
import { sleep } from './utils';
export class TurnBasedGameStore {
    constructor(turnBasedGameService, profilesStore) {
        this.turnBasedGameService = turnBasedGameService;
        this.profilesStore = profilesStore;
        _TurnBasedGameStore_gamesByEntryHash.set(this, writable({}));
        this.myGames = derived(__classPrivateFieldGet(this, _TurnBasedGameStore_gamesByEntryHash, "f"), games => {
            const myGames = {};
            for (const [hash, game] of Object.entries(games)) {
                if (game.entry.players.includes(this.myAgentPubKey)) {
                    myGames[hash] = game.entry;
                }
            }
            return myGames;
        });
        this.turnBasedGameService.cellClient.addSignalHandler(signal => {
            if (signal.data.payload.type === 'GameStarted') {
                this.handleNewGameStarted(signal.data.payload.game_hash, signal.data.payload.game_entry);
            }
            else if (signal.data.payload.type === 'NewMove') {
                this.handleNewMove(signal.data.payload.header_hash, signal.data.payload.game_move_entry);
            }
            else if (signal.data.payload.type === 'RemovedCurrentGame') {
                this.handleRemovedCurrentGame(signal.data.payload.game_hash);
            }
        });
    }
    game(gameHash) {
        return derived(__classPrivateFieldGet(this, _TurnBasedGameStore_gamesByEntryHash, "f"), games => games[gameHash]);
    }
    get myAgentPubKey() {
        return serializeHash(this.turnBasedGameService.cellClient.cellId[1]);
    }
    opponent(game) {
        return game.players.find(p => p !== this.myAgentPubKey);
    }
    /** Backend actions */
    async fetchGame(gameHash) {
        // Game entries can't change, if we have it cached do nothing
        const games = get(__classPrivateFieldGet(this, _TurnBasedGameStore_gamesByEntryHash, "f"));
        if (games[gameHash])
            return;
        const game = await this.turnBasedGameService.getGame(gameHash);
        // We asume that they are going to need the profiles
        await this.profilesStore.fetchAgentsProfiles(game.players);
        __classPrivateFieldGet(this, _TurnBasedGameStore_gamesByEntryHash, "f").update(games => {
            games[gameHash] = {
                entry: game,
                moves: [],
            };
            return games;
        });
    }
    async fetchMyCurrentGames() {
        const myCurrentGames = await this.turnBasedGameService.getMyCurrentGames();
        const opponents = Object.values(myCurrentGames).map(game => this.opponent(game));
        await this.profilesStore.fetchAgentsProfiles(opponents);
        __classPrivateFieldGet(this, _TurnBasedGameStore_gamesByEntryHash, "f").update(games => {
            // TODO: fix when we fetch more games other than our own
            games = {};
            for (const [hash, game] of Object.entries(myCurrentGames)) {
                games[hash] = {
                    entry: game,
                    moves: [],
                };
            }
            return games;
        });
    }
    async makeMove(gameHash, move) {
        const game = get(__classPrivateFieldGet(this, _TurnBasedGameStore_gamesByEntryHash, "f"))[gameHash];
        if (!game)
            throw new Error('Error making a move: game has not been fetched yet');
        const newMoveIndex = game.moves.length;
        const previousMove = game.moves[newMoveIndex - 1];
        const previousMoveHash = previousMove
            ? previousMove.header_hash
            : undefined;
        const move_entry = {
            author_pub_key: this.myAgentPubKey,
            game_hash: gameHash,
            game_move: move,
            previous_move_hash: previousMoveHash,
        };
        const m = {
            header_hash: undefined,
            game_move_entry: move_entry,
        };
        __classPrivateFieldGet(this, _TurnBasedGameStore_gamesByEntryHash, "f").update(games => {
            games[gameHash].moves.push(m);
            return games;
        });
        let header_hash;
        const numRetries = 5;
        let retryCount = 0;
        while (!header_hash && retryCount < numRetries) {
            try {
                header_hash = await this.turnBasedGameService.makeMove(gameHash, previousMoveHash, move);
            }
            catch (e) {
                // Retry if we can't see previous move hash yet
                if (JSON.stringify(e).includes("can't fetch the previous move hash yet")) {
                    await sleep(1000);
                }
                else {
                    throw e;
                }
            }
            retryCount += 1;
        }
        if (!header_hash)
            throw new Error("Could not make the move since we don't see the previous move from our opponent");
        __classPrivateFieldGet(this, _TurnBasedGameStore_gamesByEntryHash, "f").update(games => {
            games[gameHash].moves[newMoveIndex].header_hash =
                header_hash;
            return games;
        });
        return header_hash;
    }
    async fetchGameMoves(gameHash) {
        const moves = await this.turnBasedGameService.getGameMoves(gameHash);
        __classPrivateFieldGet(this, _TurnBasedGameStore_gamesByEntryHash, "f").update(games => {
            games[gameHash].moves = moves.map(m => ({
                header_hash: m.header_hash,
                game_move_entry: this.decodeMove(m.game_move_entry),
            }));
            return games;
        });
    }
    async handleNewGameStarted(entryHash, gameEntry) {
        await this.profilesStore.fetchAgentsProfiles([this.opponent(gameEntry)]);
        __classPrivateFieldGet(this, _TurnBasedGameStore_gamesByEntryHash, "f").update(games => {
            games[entryHash] = {
                entry: gameEntry,
                moves: [],
            };
            return games;
        });
    }
    async handleNewMove(moveHeaderHash, gameMove) {
        const games = get(__classPrivateFieldGet(this, _TurnBasedGameStore_gamesByEntryHash, "f"));
        if (!games[gameMove.game_hash]) {
            // We are not currently subscribing to this game
            return;
        }
        const move = this.decodeMove(gameMove);
        __classPrivateFieldGet(this, _TurnBasedGameStore_gamesByEntryHash, "f").update(games => {
            games[gameMove.game_hash].moves.push({
                header_hash: moveHeaderHash,
                game_move_entry: move,
            });
            return games;
        });
    }
    // TODO: fix when we are not only storing our games
    async handleRemovedCurrentGame(gameHash) {
        //  this.#gamesByEntryHash.update(games => {
        // delete games[gameHash];
        // return games;
        //  });
        const game = await this.turnBasedGameService.getGame(gameHash);
        __classPrivateFieldGet(this, _TurnBasedGameStore_gamesByEntryHash, "f").update(games => {
            games[gameHash] = {
                entry: game,
                moves: [],
            };
            return games;
        });
    }
    decodeMove(move) {
        return {
            ...move,
            game_move: decode(move.game_move),
        };
    }
}
_TurnBasedGameStore_gamesByEntryHash = new WeakMap();
//# sourceMappingURL=turn-based-game-store.js.map