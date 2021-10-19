import { LitElement } from 'lit';
import { Card, List, ListItem } from '@scoped-elements/material-web';
import { AgentAvatar } from '@holochain-open-dev/profiles';
import { AgentPubKeyB64 } from '@holochain-open-dev/core-types';
import { SlSkeleton } from '@scoped-elements/shoelace';
import { StoreSubscriber } from 'lit-svelte-stores';
import { EloStore } from '../elo-store';
declare const EloRanking_base: typeof LitElement & import("@open-wc/dedupe-mixin").Constructor<import("@open-wc/scoped-elements/types/src/types").ScopedElementsHost>;
export declare class EloRanking extends EloRanking_base {
    _eloStore: EloStore;
    _loading: boolean;
    _allProfiles: StoreSubscriber<import("@holochain-open-dev/core-types").Dictionary<import("@holochain-open-dev/profiles").Profile>>;
    _eloRanking: StoreSubscriber<{
        agentPubKey: string;
        elo: number;
    }[]>;
    firstUpdated(): Promise<void>;
    renderPlayer(agentPubKey: AgentPubKeyB64, elo: number): import("lit-html").TemplateResult<1>;
    renderSkeleton(): import("lit-html").TemplateResult<1>;
    render(): import("lit-html").TemplateResult<1>;
    static get scopedElements(): {
        'sl-skeleton': typeof SlSkeleton;
        'agent-avatar': typeof AgentAvatar;
        'mwc-card': typeof Card;
        'mwc-list': typeof List;
        'mwc-list-item': typeof ListItem;
    };
    static get styles(): import("lit").CSSResult[];
}
export {};
