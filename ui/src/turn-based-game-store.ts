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
import { sleep } from './utils';

export interface GameState<M> {
  entry: GameEntry;
  moves: Array<MoveInfo<M>>;
}

export class TurnBasedGameStore<M> {
  #gamesByEntryHash: Writable<Dictionary<GameState<M>>> = writable({});

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
          signal.data.payload.game_entry
        );
      } else if (signal.data.payload.type === 'NewMove') {
        this.handleNewMove(
          signal.data.payload.header_hash,
          signal.data.payload.game_move_entry
        );
      } else if (signal.data.payload.type === 'RemovedCurrentGame') {
        this.handleRemovedCurrentGame(signal.data.payload.game_hash);
      }
    });
  }

  opponent(game: GameEntry): AgentPubKeyB64 {
    return game.players.find(p => p !== this.myAgentPubKey) as AgentPubKeyB64;
  }

  /** Backend actions */

  async fetchGame(gameHash: EntryHashB64) {
    // Game entries can't change, if we have it cached do nothing
    const games = get(this.#gamesByEntryHash);

    if (games[gameHash]) return;

    const game = await this.turnBasedGameService.getGame(gameHash);

    // We asume that they are going to need the profiles
    await this.profilesStore.fetchAgentsProfiles(game.players);

    this.#gamesByEntryHash.update(games => {
      games[gameHash] = {
        entry: game,
        moves: [],
      };

      return games;
    });
  }

  async fetchMyCurrentGames() {
    const myCurrentGames = await this.turnBasedGameService.getMyCurrentGames();

    const opponents = Object.values(myCurrentGames).map(game =>
      this.opponent(game)
    );

    await this.profilesStore.fetchAgentsProfiles(opponents);

    this.#gamesByEntryHash.update(games => {
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

  async makeMove(gameHash: EntryHashB64, move: M): Promise<HeaderHashB64> {
    const game = get(this.#gamesByEntryHash)[gameHash];

    if (!game)
      throw new Error('Error making a move: game has not been fetched yet');

    const newMoveIndex = game.moves.length;
    const previousMove = game.moves[newMoveIndex - 1];
    const previousMoveHash = previousMove
      ? previousMove.header_hash
      : undefined;

    const move_entry: GameMoveEntry<M> = {
      author_pub_key: this.myAgentPubKey,
      game_hash: gameHash,
      game_move: move,
      previous_move_hash: previousMoveHash,
    };
    const m: MoveInfo<M> = {
      header_hash: undefined as any,
      game_move_entry: move_entry,
    };

    this.#gamesByEntryHash.update(games => {
      games[gameHash].moves.push(m);
      return games;
    });

    let header_hash: HeaderHashB64 | undefined;

    const numRetries = 10;
    let retryCount = 0;

    while (!header_hash && retryCount < numRetries) {
      try {
        header_hash = await this.turnBasedGameService.makeMove(
          gameHash,
          previousMoveHash,
          move
        );
      } catch (e) {
        // Retry if we can't see previous move hash yet
          console.log(JSON.stringify(e))
          //JSON.stringify(e).includes("can't fetch the previous move hash yet")
          await sleep(1000);
      }
      retryCount += 1;
    }

    if (!header_hash) {
      this.#gamesByEntryHash.update(games => {
        games[gameHash].moves.pop();
        return games;
      });
      throw new Error(
        "Could not make the move since we don't see the previous move from our opponent"
      );
    } else {
      this.#gamesByEntryHash.update(games => {
        games[gameHash].moves[newMoveIndex].header_hash =
          header_hash as HeaderHashB64;
        return games;
      });
    }
    return header_hash;
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

  // TODO: fix when we are not only storing our games
  private async handleRemovedCurrentGame(gameHash: EntryHashB64) {
    //  this.#gamesByEntryHash.update(games => {
    // delete games[gameHash];
    // return games;
    //  });
    // const game = await this.turnBasedGameService.getGame(gameHash);
    // this.#gamesByEntryHash.update(games => {
    //   games[gameHash] = {
    //     entry: game,
    //    moves: [],
    //  };
    //  return games;
    //  });
  }

  private decodeMove(move: GameMoveEntry<any>): GameMoveEntry<M> {
    return {
      ...move,
      game_move: decode(move.game_move) as any,
    };
  }
}
