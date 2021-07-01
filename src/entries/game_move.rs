use hdk::prelude::*;
use hdk::prelude::holo_hash::{AgentPubKeyB64, EntryHashB64};
use std::convert::TryFrom;

use crate::turn_based_game::TurnBasedGame;

use super::game::{
    handlers::{apply_move, build_game_state},
    GameEntry,
};

pub mod handlers;

#[hdk_entry(id = "game_move_entry")]
#[derive(Clone)]
pub struct GameMoveEntry {
    pub game_hash: EntryHashB64,
    pub author_pub_key: AgentPubKeyB64,
    pub game_move: SerializedBytes,
    pub previous_move_hash: Option<EntryHashB64>,
}

/**
 * Validate that it's the turn of the author of the move
 */
fn validate_it_is_authors_turn(
    author_pub_key: &AgentPubKeyB64,
    maybe_last_move: &Option<&GameMoveEntry>,
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

/**
 * Validates the move, getting the game
 */
pub fn validate_game_move_entry<G, M>(
    validate_data: ValidateData,
) -> ExternResult<ValidateCallbackResult>
where
    G: TurnBasedGame<M>,
    M: TryFrom<SerializedBytes>,
{
    let author = validate_data.element.header().author();

    let move_entry: GameMoveEntry = validate_data
        .element
        .entry()
        .to_app_option()?
        .ok_or(WasmError::Guest("Bad move entry content".into()))?;

    if author.clone() != AgentPubKey::from(move_entry.author_pub_key.clone()) {
        return Err(WasmError::Guest(
            "This move is not signed by its author".into(),
        ));
    }

    let maybe_element = get(
        EntryHash::from(move_entry.game_hash.clone()),
        GetOptions::default(),
    )?;

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

        let mut maybe_last_move_hash: Option<EntryHashB64> = move_entry.previous_move_hash.clone();
        let mut ordered_moves: Vec<GameMoveEntry> = Vec::new();

        while let Some(last_move_hash) = maybe_last_move_hash {
            let maybe_move_element = get(
                EntryHash::from(last_move_hash.clone()),
                GetOptions::content(),
            )?;

            if let Some(move_element) = maybe_move_element {
                let game_move: GameMoveEntry = move_element
                    .entry()
                    .to_app_option()?
                    .ok_or(WasmError::Guest("Malformed last move".into()))?;

                maybe_last_move_hash = game_move.previous_move_hash.clone();
                ordered_moves.push(game_move);
            } else {
                return Ok(ValidateCallbackResult::UnresolvedDependencies(vec![
                    EntryHash::from(last_move_hash).into(),
                ]));
            }
        }

        ordered_moves.reverse();

        let maybe_last_move = ordered_moves.last();

        validate_it_is_authors_turn(&move_entry.author_pub_key, &maybe_last_move, &game.players)?;

        let mut game_state = build_game_state::<G, M>(&game, &ordered_moves)?;

        // Get the winner
        let winner = game_state.get_winner(&game.players);

        if let Some(winner_address) = winner {
            return Err(WasmError::Guest(format!(
                "Game is already finished: {} is the winner",
                winner_address
            )));
        }

        apply_move(&mut game_state, &game, &move_entry)?;

        Ok(ValidateCallbackResult::Valid)
    } else {
        return Ok(ValidateCallbackResult::UnresolvedDependencies(vec![
            EntryHash::from(move_entry.game_hash).into(),
        ]));
    }
}
