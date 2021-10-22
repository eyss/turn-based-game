use std::collections::HashMap;

use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::{
    game::{build_game_state, get_game, verify_we_see_previous_move_hash},
    game_move::{GameMoveEntry, MoveInfo},
    signal::{self, SignalPayload},
    turn_based_game::TurnBasedGame,
};

/** Public handlers */

/**
 * Creates the next move for the given game, linking the game to the move
 * If this is the first move, we should
 */
pub fn create_move<G: TurnBasedGame>(
    game_hash: EntryHashB64,
    previous_move_hash: Option<HeaderHashB64>,
    game_move: G::GameMove,
) -> ExternResult<HeaderHashB64> {
    let moves = get_moves_entries(game_hash.clone())?;
    verify_we_see_previous_move_hash(&moves, previous_move_hash.clone())?;

    let game = get_game(game_hash.clone())?;
    let only_moves: Vec<GameMoveEntry> = moves.iter().map(|m| m.1.clone()).collect();

    let game_state = build_game_state::<G>(&game, &only_moves)?;

    let move_bytes: SerializedBytes = game_move
        .clone()
        .try_into()
        .or(Err(WasmError::Guest("Couldn't serialize game move".into())))?;

    let new_game_state = G::apply_move(
        game_state,
        game_move,
        agent_info()?.agent_latest_pubkey.into(),
    )?;

    let game_state_bytes: SerializedBytes = new_game_state.try_into().or(Err(WasmError::Guest(
        "Couldn't serialize game state".into(),
    )))?;

    let game_move = GameMoveEntry {
        game_hash: game_hash.clone().into(),
        author_pub_key: agent_info()?.agent_latest_pubkey.into(),
        game_move: move_bytes,
        resulting_game_state: game_state_bytes,
        previous_move_hash: previous_move_hash.clone(),
    };

    let header_hash = create_entry(&game_move)?;

    let move_hash = hash_entry(&game_move)?;

    create_link(
        EntryHash::from(game_hash.clone()),
        move_hash.clone(),
        game_to_move_tag(),
    )?;

    // Sends the newly created move to all opponents of the game
    let signal = SignalPayload::NewMove(MoveInfo {
        header_hash: header_hash.clone().into(),
        game_move_entry: game_move,
    });

    signal::send_signal_to_players(game, signal)?;

    Ok(header_hash.into())
}

/**
 * Get all the moves for the given game
 */
pub fn get_game_moves(game_hash: EntryHashB64) -> ExternResult<Vec<MoveInfo>> {
    let moves = get_moves_entries(game_hash)?;

    Ok(moves
        .into_iter()
        .map(|(header_hash, move_entry)| MoveInfo {
            header_hash,
            game_move_entry: move_entry,
        })
        .collect())
}

/**
 * Returns all the moves for the given game
 */
pub fn get_moves_entries(
    game_hash: EntryHashB64,
) -> ExternResult<Vec<(HeaderHashB64, GameMoveEntry)>> {
    let links = get_links(EntryHash::from(game_hash), Some(game_to_move_tag()))?;

    let get_inputs = links
        .into_inner()
        .into_iter()
        .map(|link| GetInput::new(link.target.into(), GetOptions::default()))
        .collect();

    let get_results = HDK.with(|hdk| hdk.borrow().get(get_inputs))?;

    let mut moves = get_results
        .into_iter()
        .map(|maybe_element| {
            let element = maybe_element.ok_or(WasmError::Guest("Couldn't get move".into()))?;
            let move_entry = element
                .entry()
                .to_app_option()?
                .ok_or(WasmError::Guest("Couldn't deserialize move".into()))?;

            Ok((element.header_address().clone().into(), move_entry))
        })
        .collect::<ExternResult<Vec<(HeaderHashB64, GameMoveEntry)>>>()?;

    order_moves(&mut moves)
}

/** Private helpers */

/**
 * Returns the moves ordered following the previous_move_address
 *
 * Returns error if in any case the chain of moves is not valid
 */
fn order_moves(
    moves: &mut Vec<(HeaderHashB64, GameMoveEntry)>,
) -> ExternResult<Vec<(HeaderHashB64, GameMoveEntry)>> {
    if moves.is_empty() {
        return Ok(vec![]);
    }

    // previous_move_hash -> next_move_hash
    let mut next_moves_map: HashMap<HeaderHashB64, HeaderHashB64> = HashMap::new();
    // move_hash -> move_entry
    let mut moves_map: HashMap<HeaderHashB64, GameMoveEntry> = HashMap::new();

    let mut first_move: Option<HeaderHashB64> = None;

    for move_entry in moves {
        if let Some(previous_move) = move_entry.1.previous_move_hash.clone() {
            if next_moves_map.contains_key(&previous_move) {
                return Err(WasmError::Guest(
                    "There are two moves pointing to the same next move".into(),
                ));
            }

            next_moves_map.insert(previous_move, move_entry.0.clone());
        } else {
            if let Some(_) = first_move {
                return Err(WasmError::Guest(
                    "There are two first moves in this list".into(),
                ));
            }
            first_move = Some(move_entry.0.clone());
        }

        if moves_map.contains_key(&move_entry.0) {
            return Err(WasmError::Guest(
                "There are two entries with the same hash in this list".into(),
            ));
        }

        moves_map.insert(move_entry.0.clone(), move_entry.1.clone());
    }

    match first_move {
        None => {
            return Err(WasmError::Guest(
                "There is no first move in this list".into(),
            ))
        }
        Some(first_move_hash) => {
            let mut ordered_moves: Vec<(HeaderHashB64, GameMoveEntry)> = vec![];

            let mut maybe_next_move_hash: Option<HeaderHashB64> = Some(first_move_hash);

            while let Some(next_move_hash) = maybe_next_move_hash {
                match moves_map.get(&next_move_hash) {
                    None => Err(WasmError::Guest(
                        "There are missing moves in the list".into(),
                    )),
                    Some(move_entry) => {
                        ordered_moves.push((next_move_hash.clone(), move_entry.clone()));
                        Ok(())
                    }
                }?;

                maybe_next_move_hash = next_moves_map.get(&next_move_hash).cloned();
            }

            Ok(ordered_moves)
        }
    }
}

fn game_to_move_tag() -> LinkTag {
    LinkTag::from(String::from("game->move").as_bytes().to_vec())
}
