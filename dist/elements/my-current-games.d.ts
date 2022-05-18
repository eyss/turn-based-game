import { LitElement } from 'lit';
import { AgentPubKeyB64 } from '@holochain-open-dev/core-types';
import { Card, List, ListItem, Icon, Button, CircularProgress } from '@scoped-elements/material-web';
import { StoreSubscriber } from 'lit-svelte-stores';
import { AgentAvatar } from '@holochain-open-dev/profiles';
import { TurnBasedGameStore } from '../turn-based-game-store';
declare const MyCurrentGames_base: typeof LitElement & import("@open-wc/dedupe-mixin").Constructor<import("@open-wc/scoped-elements/types/src/types").ScopedElementsHost>;
export declare class MyCurrentGames extends MyCurrentGames_base {
    _store: TurnBasedGameStore<any>;
    loading: boolean;
    _knownProfiles: StoreSubscriber<Record<string, import("@holochain-open-dev/profiles").Profile>>;
    _myGames: StoreSubscriber<import("@holochain-open-dev/core-types").Dictionary<import("..").GameEntry>>;
    nicknameOf(agent: AgentPubKeyB64): string;
    firstUpdated(): Promise<void>;
    renderGames(): import("lit-html").TemplateResult<1>;
    render(): import("lit-html").TemplateResult<1>;
    static get scopedElements(): {
        'agent-avatar': typeof AgentAvatar;
        'mwc-icon': typeof Icon;
        'mwc-card': typeof Card;
        'mwc-list': typeof List;
        'mwc-button': typeof Button;
        'mwc-list-item': typeof ListItem;
        'mwc-circular-progress': typeof CircularProgress;
    };
    static styles: import("lit").CSSResult[];
}
export {};
