use crate::game::{Game, GameEntry};
use crate::game_move::{Move, MoveEntry};
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

/**
 * Creates the game
 */
pub fn create_game(game: GameEntry) -> ZomeApiResult<Address> {
    hdk::commit_entry(&game.entry())
}

/**
 * Creates the next move for the given game, linking the game to the move
 */
pub fn create_move<M, G>(
    game_address: &Address,
    game_move: M,
    current_move: &Option<Address>,
) -> ZomeApiResult<Address>
where
    G: Game,
    M: Move<G>,
{
    let move_json = game_move.into();

    let game_move = MoveEntry {
        game_address: game_address.clone(),
        author_address: hdk::AGENT_ADDRESS.clone(),
        game_move: move_json,
        previous_move_address: current_move.clone(),
    };

    let move_address = hdk::commit_entry(&game_move.entry())?;
    hdk::link_entries(&game_address, &move_address, "game->move", "")?;

    Ok(move_address)
}
