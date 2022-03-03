use hc_mixin_turn_based_game::*;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

mod tictactoe;

use tictactoe::TicTacToe;

entry_defs![GameMoveEntry::entry_def(), GameEntry::entry_def()];

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    init_turn_based_games()
}

#[hdk_extern]
fn who_am_i(_: ()) -> ExternResult<AgentPubKeyB64> {
    Ok(agent_info()?.agent_latest_pubkey.into())
}

#[hdk_extern]
fn create_tictactoe_game(rival: AgentPubKeyB64) -> ExternResult<EntryHashB64> {
    let hash = create_game(vec![rival, agent_info()?.agent_latest_pubkey.into()])?;

    Ok(hash.into())
}

#[hdk_extern]
fn get_game_state(game_hash: EntryHashB64) -> ExternResult<TicTacToe> {
    hc_mixin_turn_based_game::get_game_state::<TicTacToe>(game_hash.into())
}

#[hdk_extern]
fn remove_current_game(game_hash: EntryHashB64) -> ExternResult<()> {
    hc_mixin_turn_based_game::remove_current_game(game_hash)
}

#[hdk_extern]
fn get_winner(game_hash: EntryHashB64) -> ExternResult<Option<u8>> {
    let state = hc_mixin_turn_based_game::get_game_state::<TicTacToe>(game_hash.into())?;

    Ok(state.winner())
}

mixin_turn_based_game!(TicTacToe);
