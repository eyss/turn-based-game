import { AgentPubKeyB64, HeaderHashB64 } from '@holochain-open-dev/core-types';

export interface GameEntry {
  players: Array<AgentPubKeyB64>;
  created_at: number;
}

export interface GameMoveEntry<M> {
  game_hash: string;
  author_pub_key: AgentPubKeyB64;
  game_move: M;
  previous_move_hash: HeaderHashB64 | undefined;
}

export interface MoveInfo<M> {
  header_hash: string;
  game_move_entry: GameMoveEntry<M>;
}
