use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use super::GameMoveEntry;
use crate::game::{apply_move, GameEntry};
use crate::turn_based_game::{GameStatus, TurnBasedGame};

/**
* Validates the move, getting the game
*/
pub fn validate_game_move_entry<G: TurnBasedGame>(
    validate_data: ValidateData,
) -> ExternResult<ValidateCallbackResult> {
    let author = validate_data.element.header().author();

    let maybe_move_entry: Option<GameMoveEntry> = validate_data.element.entry().to_app_option()?;

    if let Some(move_entry) = maybe_move_entry {
        trace!("Validating move: {:?}", move_entry);

        if author.clone() != AgentPubKey::from(move_entry.author_pub_key.clone()) {
            return Ok(ValidateCallbackResult::Invalid(
                "This move is not signed by its author".into(),
            ));
        }

        let entry_hashed = must_get_entry(EntryHash::from(move_entry.game_hash.clone()))?;
        trace!("Validating move, game entry: {:?}", entry_hashed);

        let game: GameEntry = entry_hashed.as_content().try_into()?;

        trace!("Validating move, game entry: {:?}", game);
        if !game.players.contains(&move_entry.author_pub_key) {
            return Ok(ValidateCallbackResult::Invalid(
                "The author of the move is not playing the game".into(),
            ));
        }

        let maybe_last_move_hash: Option<HeaderHashB64> = move_entry.previous_move_hash.clone();

        let mut previous_game_state = G::initial(game.players.clone());
        let mut maybe_last_move: Option<GameMoveEntry> = None;

        if let Some(last_move_hash) = maybe_last_move_hash {
            let move_element = must_get_valid_element(last_move_hash.clone().into())?;

            let maybe_game_move: Option<GameMoveEntry> = move_element.entry().to_app_option()?;

            trace!("Validating move, previous move element: {:?}", move_element);

            if let Some(game_move) = maybe_game_move {
                trace!("Validating move, previous move entry: {:?}", game_move);
                previous_game_state = G::try_from(game_move.clone().resulting_game_state).or(
                    Err(WasmError::Guest("Couldn't deserialize game state".into())),
                )?;
                maybe_last_move = Some(game_move);
            } else {
                return Ok(ValidateCallbackResult::UnresolvedDependencies(vec![
                    HeaderHash::from(last_move_hash).into(),
                ]));
            }
        }

        validate_it_is_authors_turn(&move_entry.author_pub_key, &maybe_last_move, &game.players)?;

        // Get the outcome of the game

        if let GameStatus::Finished = previous_game_state.status() {
            return Err(WasmError::Guest(format!(
                "Game is already finished: cannot make any more moves",
            )));
        }

        apply_move(&mut previous_game_state, &move_entry)?;

        Ok(ValidateCallbackResult::Valid)
    } else {
        return Ok(ValidateCallbackResult::Invalid(
            "Game move validation was called without an entry being present".into(),
        ));
    }
}

/** Helper functions */

/**
 * Validate that it's the turn of the author of the move
 */
fn validate_it_is_authors_turn(
    author_pub_key: &AgentPubKeyB64,
    maybe_last_move: &Option<GameMoveEntry>,
    players: &Vec<AgentPubKeyB64>,
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
