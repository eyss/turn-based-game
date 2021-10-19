use chrono::{DateTime, NaiveDateTime, Utc};
use hdk::prelude::holo_hash::{AgentPubKeyB64, EntryHashB64};
use hdk::prelude::*;

use crate::{current_games, GameOutcome};
use crate::{
    game_move::{self, GameMoveEntry},
    signal::{send_signal_to_players, SignalPayload},
    turn_based_game::TurnBasedGame,
};

use super::GameEntry;

/** Public handlers */

/**
 * Creates the game
 */
pub fn create_game(players: Vec<AgentPubKeyB64>) -> ExternResult<EntryHashB64> {
    let now = sys_time()?.as_seconds_and_nanos();

    let date_time = DateTime::from_utc(NaiveDateTime::from_timestamp(now.0, now.1), Utc);

    let game = GameEntry {
        players: players.clone(),
        created_at: date_time,
    };

    create_entry(&game)?;

    let game_hash = hash_entry(&game)?;

    current_games::add_current_game(game_hash.clone(), players)?;

    let game_hash_b64 = EntryHashB64::from(game_hash);

    let signal = SignalPayload::GameStarted {
        game_hash: game_hash_b64.clone(),
        game_entry: game.clone(),
    };

    send_signal_to_players(game, signal)?;

    Ok(game_hash_b64)
}

/**
 * Gets the winner of the game
 */
pub fn get_game_outcome<G: TurnBasedGame>(
    game_hash: EntryHashB64,
) -> ExternResult<GameOutcome<G::GameResult>> {
    let game = get_game(game_hash.clone())?;
    let game_state = get_game_state::<G>(game_hash)?;

    Ok(game_state.outcome(game.players.clone()))
}

/**
 * Gets the current state of the game
 */
pub fn get_game_state<G: TurnBasedGame>(game_hash: EntryHashB64) -> ExternResult<G> {
    let moves = game_move::handlers::get_moves_entries(game_hash.clone())?;
    let game = get_game(game_hash.clone())?;
    let only_moves: Vec<GameMoveEntry> = moves.iter().map(|m| m.1.clone()).collect();

    build_game_state::<G>(&game, &only_moves)
}

pub fn get_game(game_hash: EntryHashB64) -> ExternResult<GameEntry> {
    let element = get(EntryHash::from(game_hash), GetOptions::default())?
        .ok_or(WasmError::Guest("There is no game at this hash".into()))?;

    element
        .entry()
        .to_app_option()?
        .ok_or(WasmError::Guest("Couldn't deserialize game entry".into()))
}

pub(crate) fn build_game_state<G: TurnBasedGame>(
    game_entry: &GameEntry,
    moves: &Vec<GameMoveEntry>,
) -> ExternResult<G> {
    let mut game_state = G::initial(&game_entry.players.clone());

    for (_index, game_move) in moves.iter().enumerate() {
        apply_move(&mut game_state, &game_entry, game_move)?;
    }
    return Ok(game_state);
}

pub(crate) fn apply_move<G: TurnBasedGame>(
    game_state: &mut G,
    game_entry: &GameEntry,
    game_move: &GameMoveEntry,
) -> ExternResult<()> {
    let move_content = G::GameMove::try_from(game_move.game_move.clone())
        .or(Err(WasmError::Guest("Couldnt't convert game move".into())))?;

    game_state.apply_move(
        move_content,
        game_move.author_pub_key.clone(),
        game_entry.players.clone(),
    )?;

    Ok(())
}
