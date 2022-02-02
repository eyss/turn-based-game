import { html, LitElement } from 'lit';
import { state } from 'lit/decorators.js';
import { contextProvided } from '@holochain-open-dev/context';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { AgentPubKeyB64 } from '@holochain-open-dev/core-types';

import {
  Card,
  List,
  ListItem,
  Icon,
  Button,
  CircularProgress,
} from '@scoped-elements/material-web';
import { StoreSubscriber } from 'lit-svelte-stores';
import { AgentAvatar } from '@holochain-open-dev/profiles';

import { sharedStyles } from '../shared-styles';
import { turnBasedGameStoreContext } from '../context';
import { TurnBasedGameStore } from '../turn-based-game-store';

export class MyCurrentGames extends ScopedElementsMixin(LitElement) {
  @contextProvided({ context: turnBasedGameStoreContext })
  _store!: TurnBasedGameStore<any>;

  @state()
  loading = true;

  _knownProfiles = new StoreSubscriber(
    this,
    () => this._store.profilesStore.knownProfiles
  );

  _myGames = new StoreSubscriber(this, () => this._store.myGames);

  nicknameOf(agent: AgentPubKeyB64) {
    return this._knownProfiles.value[agent].nickname;
  }

  async firstUpdated() {
    await this._store.fetchMyCurrentGames();

    this.loading = false;
  }

  renderGames() {
    if (Object.keys(this._myGames.value).length === 0)
      return html`<div class="column center-content" style="flex: 1;">
        <span class="placeholder" style="margin: 16px;"
          >You are not playing any game at the moment</span
        >
      </div>`;

    return html`<div class="flex-scrollable-parent">
      <div class="flex-scrollable-container">
        <div class="flex-scrollable-y">
          <mwc-list>
            ${Object.entries(this._myGames.value).map(
              ([hash, game]) =>
                html` <div class="row center-content">
                  <mwc-list-item
                    hasMeta
                    twoline
                    style="flex: 1;"
                    graphic="avatar"
                    @click=${() =>
                      this.dispatchEvent(
                        new CustomEvent('open-game', {
                          detail: {
                            gameHash: hash,
                          },
                          composed: true,
                          bubbles: true,
                        })
                      )}
                  >
                    <agent-avatar
                      slot="graphic"
                      .agentPubKey=${this._store.opponent(game)}
                    ></agent-avatar>

                    <span>${this.nicknameOf(this._store.opponent(game))} </span>
                    <span slot="secondary"
                      >Started at
                      ${new Date(game.created_at).toLocaleString()}</span
                    >

                    <mwc-icon slot="meta">launch</mwc-icon>
                  </mwc-list-item>
                </div>`
            )}
          </mwc-list>
        </div>
      </div>
    </div>`;
  }

  render() {
    if (!this._myGames.value)
      return html`<div class="container">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`;

    return html`
      <mwc-card style="flex: 1; min-width: 270px;">
        <div class="column" style="margin: 16px; flex: 1;">
          <span class="title">Current Games</span>
          ${this.renderGames()}
        </div>
      </mwc-card>
    `;
  }

  static get scopedElements() {
    return {
      'agent-avatar': AgentAvatar,
      'mwc-icon': Icon,
      'mwc-card': Card,
      'mwc-list': List,
      'mwc-button': Button,
      'mwc-list-item': ListItem,
      'mwc-circular-progress': CircularProgress,
    };
  }

  static styles = [sharedStyles];
}
