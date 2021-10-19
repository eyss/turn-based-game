use hc_mixin_turn_based_game::*;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

mod tictactoe;

use tictactoe::{Piece, TicTacToe, TicTacToeMove, Winner};

entry_defs![GameMoveEntry::entry_def(), GameEntry::entry_def()];

#[hdk_extern]
fn who_am_i(_: ()) -> ExternResult<AgentPubKeyB64> {
    Ok(agent_info()?.agent_latest_pubkey.into())
}

#[hdk_extern]
fn create_tictactoe_game(rival: AgentPubKeyB64) -> ExternResult<EntryHashB64> {
    let hash = create_game(vec![rival, agent_info()?.agent_latest_pubkey.into()])?;

    Ok(hash.into())
}

#[derive(Serialize, Deserialize, Debug)]
struct PlacePieceInput {
    game_hash: EntryHashB64,
    previous_move_hash: Option<HeaderHashB64>,
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
) -> ExternResult<HeaderHashB64> {
    let game_move = TicTacToeMove::Place(Piece { x, y });
    let move_hash = create_move::<TicTacToe>(
        game_hash.into(),
        previous_move_hash.map(|hash| hash.into()),
        game_move,
    )?;
    Ok(move_hash)
}

#[hdk_extern]
fn get_outcome(game_hash: EntryHashB64) -> ExternResult<GameOutcome<Winner>> {
    let winner = hc_mixin_turn_based_game::get_game_outcome::<TicTacToe>(game_hash.into())?;

    Ok(winner)
}

mixin_turn_based_game!(TicTacToe);
