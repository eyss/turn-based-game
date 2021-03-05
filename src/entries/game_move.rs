use hdk::prelude::*;
use std::convert::TryFrom;

use crate::turn_based_game::TurnBasedGame;

use self::handlers::get_moves_entries;

use super::game::GameEntry;

pub mod handlers;

#[hdk_entry(id = "game_move")]
#[derive(Clone)]
pub struct MoveEntry {
    pub game_hash: EntryHash,
    pub author_pub_key: AgentPubKey,
    pub game_move: SerializedBytes,
    pub previous_move_hash: Option<EntryHash>,
}

/**
 * Validate that it's the turn of the author of the move
 */
 fn validate_it_is_authors_turn(
    author_pub_key: &AgentPubKey,
    maybe_last_move: &Option<&MoveEntry>,
    players: &Vec<AgentPubKey>,
) -> ExternResult<()> {
    let maybe_last_player_index = match maybe_last_move {
        Some(last_move) => players
            .iter()
            .position(|p| p.clone() == last_move.author_pub_key),
        None => None,
    };

    // Get the index of the player whose turn it is
    let player_index = match maybe_last_player_index {
        Some(last_player_index) => {
            let new_index = last_player_index + 1;

            match new_index >= players.len() {
                true => 0,
                false => new_index,
            }
        }
        None => 0,
    };

    if players[player_index] != author_pub_key.clone() {
        return Err(WasmError::Guest(
            "It's not the turn of the author of the move".into(),
        ));
    }

    Ok(())
}

/**
 * Validates the move, getting the game
 */
pub fn validate_move<G, M>(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult>
where
    G: TurnBasedGame<M>,
    M: TryFrom<SerializedBytes>,
{
    let author = validate_data.element.header().author();

    let move_entry: MoveEntry = validate_data
        .element
        .entry()
        .to_app_option()?
        .ok_or(WasmError::Guest("Bad move entry content".into()))?;

    if author.clone() != move_entry.author_pub_key {
        return Err(WasmError::Guest(
            "This move is not signed by its author".into(),
        ));
    }

    let maybe_element = get(move_entry.game_hash.clone(), GetOptions::default())?;

    if let Some(element) = maybe_element {
        let game: GameEntry = element
            .entry()
            .to_app_option()?
            .ok_or(WasmError::Guest("Bad game entry content".into()))?;

        if !game.players.contains(&move_entry.author_pub_key) {
            return Err(WasmError::Guest(
                "The author of the move is not playing the game".into(),
            ));
        }

        let ordered_moves: Vec<MoveEntry> = get_moves_entries(move_entry.game_hash)?;

        let maybe_last_move = ordered_moves.last();

        validate_it_is_authors_turn(&move_entry.author_pub_key, &maybe_last_move, &game.players)?;

        let mut game_state = G::initial(&game.players.clone());

        for (index, game_move) in ordered_moves.iter().enumerate() {
            let move_content = M::try_from(game_move.game_move.clone())
                .or(Err(WasmError::Guest("Could not deserialize move".into())))?;
            game_state.apply_move(&move_content, &game_move.author_pub_key)?;
        }

        // Get the winner
        let winner = game_state.get_winner(&game.players);

        if let Some(winner_address) = winner {
            return Err(WasmError::Guest(format!(
                "Game is already finished: {} is the winner",
                winner_address
            )));
        }

        let new_move = M::try_from(move_entry.game_move.clone())
            .or(Err(WasmError::Guest("Could not deserialize move".into())))?;

        game_state.apply_move(&new_move, &move_entry.author_pub_key)?;

        Ok(ValidateCallbackResult::Valid)
    } else {
        return Ok(ValidateCallbackResult::UnresolvedDependencies(vec![
            move_entry.game_hash.into(),
        ]));
    }
}
