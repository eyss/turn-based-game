import { Context, createContext } from '@lit-labs/context';
import { TurnBasedGameStore } from './turn-based-game-store';

export const turnBasedGameStoreContext: Context<TurnBasedGameStore<any>> =
  createContext('hc_mixin_turn_based_game/store');
