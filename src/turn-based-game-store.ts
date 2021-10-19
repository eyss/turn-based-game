import {
  AgentPubKeyB64,
  Dictionary,
  EntryHashB64,
  HeaderHashB64,
  serializeHash,
} from '@holochain-open-dev/core-types';
import { ProfilesStore } from '@holochain-open-dev/profiles';
import { decode } from '@msgpack/msgpack';
import { derived, get, writable, Writable } from 'svelte/store';
import { TurnBasedGameService } from './turn-based-game-service';
import { GameEntry, GameMoveEntry, MoveInfo } from './types';

export interface GameState<M> {
  entry: GameEntry;
  moves: Array<MoveInfo<M>>;
}

export class TurnBasedGameStore<M> {
  #gamesByEntryHash: Writable<{
    [key: string]: GameState<M>;
  }> = writable({});

  public game(gameHash: EntryHashB64) {
    return derived(this.#gamesByEntryHash, games => games[gameHash]);
  }

  public myGames = derived(this.#gamesByEntryHash, games => {
    const myGames: Dictionary<GameEntry> = {};

    for (const [hash, game] of Object.entries(games)) {
      if (game.entry.players.includes(this.myAgentPubKey)) {
        myGames[hash] = game.entry;
      }
    }

    return myGames;
  });

  public get myAgentPubKey() {
    return serializeHash(this.turnBasedGameService.cellClient.cellId[1]);
  }

  constructor(
    protected turnBasedGameService: TurnBasedGameService,
    public profilesStore: ProfilesStore
  ) {
    this.turnBasedGameService.cellClient.addSignalHandler(signal => {
      if (signal.data.payload.type === 'GameStarted') {
        this.handleNewGameStarted(
          signal.data.payload.game_hash,
          signal.data.payload.game_hash
        );
      } else if (signal.data.payload.type === 'NewMove') {
        this.handleNewMove(
          signal.data.payload.header_hash,
          signal.data.payload.game_move_entry
        );
      }
    });
  }

  opponent(game: GameEntry): AgentPubKeyB64 {
    return game.players.find(p => p !== this.myAgentPubKey) as AgentPubKeyB64;
  }

  /** Backend actions */

  async fetchMyCurrentGames() {
    const myCurrentGames = await this.turnBasedGameService.getMyCurrentGames();

    const opponents = Object.values(myCurrentGames).map(game =>
      this.opponent(game)
    );

    await this.profilesStore.fetchAgentsProfiles(opponents);

    this.#gamesByEntryHash.update(games => {
      for (const [hash, game] of Object.entries(myCurrentGames)) {
        games[hash] = {
          entry: game,
          moves: [],
        };
      }

      return games;
    });
  }

  async fetchGameMoves(gameHash: EntryHashB64) {
    const moves = await this.turnBasedGameService.getGameMoves(gameHash);

    this.#gamesByEntryHash.update(games => {
      games[gameHash].moves = moves.map(m => ({
        header_hash: m.header_hash,
        game_move_entry: this.decodeMove(m.game_move_entry),
      }));

      return games;
    });
  }

  private async handleNewGameStarted(
    entryHash: EntryHashB64,
    gameEntry: GameEntry
  ) {
    await this.profilesStore.fetchAgentsProfiles([this.opponent(gameEntry)]);

    this.#gamesByEntryHash.update(games => {
      games[entryHash] = {
        entry: gameEntry,
        moves: [],
      };
      return games;
    });
  }

  private async handleNewMove(
    moveHeaderHash: HeaderHashB64,
    gameMove: GameMoveEntry<M>
  ) {
    const games = get(this.#gamesByEntryHash);

    if (!games[gameMove.game_hash]) {
      // We are not currently subscribing to this game
      return;
    }

    const move = this.decodeMove(gameMove);

    this.#gamesByEntryHash.update(games => {
      games[gameMove.game_hash].moves.push({
        header_hash: moveHeaderHash,
        game_move_entry: move,
      });

      return games;
    });
  }

  private decodeMove(move: GameMoveEntry<any>): GameMoveEntry<M> {
    return {
      ...move,
      game_move: decode(move.game_move) as any,
    };
  }
}
