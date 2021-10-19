import { Context, createContext } from '@lit-labs/context';
import { EloStore } from './elo-store';

export const eloStoreContext: Context<EloStore> =
  createContext('hc_mixin_elo/store');
