import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { html, LitElement } from 'lit';
import { state } from 'lit/decorators.js';
import { Card, List, ListItem } from '@scoped-elements/material-web';
import { contextProvided } from '@lit-labs/context';
import { AgentAvatar } from '@holochain-open-dev/profiles';
import { AgentPubKeyB64 } from '@holochain-open-dev/core-types';
import { SlSkeleton, lightTheme } from '@scoped-elements/shoelace';
import { StoreSubscriber } from 'lit-svelte-stores';

import { eloStoreContext } from '../context';
import { EloStore } from '../elo-store';
import { sharedStyles } from '../shared-styles';

export class EloRanking extends ScopedElementsMixin(LitElement) {
  @contextProvided({ context: eloStoreContext })
  _eloStore!: EloStore;

  @state()
  _loading = true;

  _allProfiles = new StoreSubscriber(
    this,
    () => this._eloStore.profilesStore.knownProfiles
  );

  _eloRanking = new StoreSubscriber(this, () => this._eloStore.eloRanking);

  async firstUpdated() {
    await this._eloStore.profilesStore.fetchAllProfiles();
    const allPubKeys = Object.keys(this._allProfiles.value);
    await this._eloStore.fetchEloForAgents(allPubKeys);

    this._loading = false;
  }

  renderPlayer(agentPubKey: AgentPubKeyB64, elo: number) {
    const profile = this._allProfiles.value[agentPubKey];

    return html`
      <mwc-list-item
        graphic="avatar"
        hasMeta
        .value=${agentPubKey}
        style="--mdc-list-item-graphic-size: 32px;"
      >
        <agent-avatar slot="graphic" .agentPubKey=${agentPubKey}>
        </agent-avatar>
        <span>${profile ? profile.nickname : agentPubKey}</span>
        <span slot="meta" style="color: black; font-size: 16px;">${elo}</span>
      </mwc-list-item>
    `;
  }

  renderSkeleton() {
    return html` <div class="column" style="margin-top: 8px; margin-left: 4px;">
      ${[0, 1, 2].map(
        () => html`
          <div class="row" style="align-items: center; margin: 8px;">
            <sl-skeleton
              effect="sheen"
              style="width: 32px; height: 32px; margin-right: 16px;"
            ></sl-skeleton>

            <sl-skeleton
              effect="sheen"
              style="width: 200px; height: 16px;"
            ></sl-skeleton>
          </div>
        `
      )}
    </div>`;
  }

  render() {
    return html`
      <mwc-card style="flex: 1; min-width: 270px;">
        <div class="column" style="margin: 16px; flex: 1;">
          <span class="title">ELO Ranking</span>
          ${this._loading
            ? this.renderSkeleton()
            : html`
                <mwc-list noninteractive>
                  ${this._eloRanking.value.map(e =>
                    this.renderPlayer(e.agentPubKey, e.elo)
                  )}
                </mwc-list>
              `}
        </div>
      </mwc-card>
    `;
  }

  static get scopedElements() {
    return {
      'sl-skeleton': SlSkeleton,
      'agent-avatar': AgentAvatar,
      'mwc-card': Card,
      'mwc-list': List,
      'mwc-list-item': ListItem,
    };
  }

  static get styles() {
    return [sharedStyles, lightTheme];
  }
}
