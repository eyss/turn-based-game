use hdk::prelude::*;
use holo_hash::{AgentPubKeyB64, EntryHashB64};
use holochain_turn_based_game::prelude::*;

mod tictactoe;

use tictactoe::{Piece, TicTacToe, TicTacToeMove};

entry_defs![GameMoveEntry::entry_def(), GameEntry::entry_def()];

#[hdk_extern]
fn who_am_i(_: ()) -> ExternResult<AgentPubKeyB64> {
    Ok(agent_info()?.agent_latest_pubkey.into())
}

#[hdk_extern]
fn create_game(rival: AgentPubKeyB64) -> ExternResult<EntryHashB64> {
    let hash = holochain_turn_based_game::prelude::create_game(vec![
        rival.into(),
        agent_info()?.agent_latest_pubkey,
    ])?;

    Ok(hash.into())
}

#[derive(Serialize, Deserialize, Debug)]
struct PlacePieceInput {
    game_hash: EntryHashB64,
    previous_move_hash: Option<EntryHashB64>,
    x: usize,
    y: usize,
}
#[hdk_extern]
fn place_piece(
    PlacePieceInput {
        game_hash,
        previous_move_hash,
        x,
        y,
    }: PlacePieceInput,
) -> ExternResult<EntryHashB64> {
    let game_move = TicTacToeMove::Place(Piece { x, y });
    let move_hash = holochain_turn_based_game::prelude::create_move(
        game_hash.into(),
        previous_move_hash.map(|hash| hash.into()),
        game_move,
    )?;
    Ok(move_hash.into())
}

#[hdk_extern]
fn get_winner(game_hash: EntryHashB64) -> ExternResult<Option<AgentPubKeyB64>> {
    let winner = holochain_turn_based_game::prelude::get_game_winner::<TicTacToe, TicTacToeMove>(
        game_hash.into(),
    )?;

    Ok(winner.map(|w| w.into()))
}

#[hdk_extern]
fn get_game_state(game_hash: EntryHashB64) -> ExternResult<TicTacToe> {
    holochain_turn_based_game::prelude::get_game_state::<TicTacToe, TicTacToeMove>(game_hash.into())
}

#[hdk_extern]
fn validate_create_entry_game_entry(
    validate_data: ValidateData,
) -> ExternResult<ValidateCallbackResult> {
    holochain_turn_based_game::prelude::validate_game_entry::<TicTacToe, TicTacToeMove>(
        validate_data,
    )
}

#[hdk_extern]
fn validate_create_entry_game_move_entry(
    validate_data: ValidateData,
) -> ExternResult<ValidateCallbackResult> {
    holochain_turn_based_game::prelude::validate_game_move_entry::<TicTacToe, TicTacToeMove>(
        validate_data,
    )
}
