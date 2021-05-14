use chrono::{DateTime, NaiveDateTime, Utc};
use game_move::GameMoveEntry;
use hdk::prelude::*;
use holo_hash::{AgentPubKeyB64, EntryHashB64};

use crate::{
    entries::game_move,
    signal::{send_signal_to_players, SignalPayload},
    turn_based_game::TurnBasedGame,
};

use super::{GameEntry, GameInfo, MoveInfo};

/** Public handlers */

/**
 * Creates the game
 */
pub fn create_game(players: Vec<AgentPubKeyB64>) -> ExternResult<EntryHash> {
    let now = sys_time()?;

    let date_time = DateTime::from_utc(
        NaiveDateTime::from_timestamp(now.as_secs() as i64, now.subsec_nanos()),
        Utc,
    );

    let game = GameEntry {
        players,
        created_at: date_time,
    };

    create_entry(&game)?;

    let game_hash = hash_entry(&game)?;

    let signal = SignalPayload::GameStarted((game_hash.clone().into(), game.clone()));

    send_signal_to_players(game, signal)?;

    Ok(game_hash)
}

/**
 * Gets the winner of the game
 */
pub fn get_game_winner<G, M>(game_hash: EntryHash) -> ExternResult<Option<AgentPubKeyB64>>
where
    G: TurnBasedGame<M>,
    M: TryFrom<SerializedBytes>,
{
    let game = get_game(game_hash.clone())?;
    let game_state = get_game_info::<G, M>(game_hash)?;

    Ok(game_state.game_state.get_winner(&game.players))
}

/**
 * Gets the current state of the game
 */
pub fn get_game_info<G, M>(game_hash: EntryHash) -> ExternResult<GameInfo<G, M>>
where
    G: TurnBasedGame<M>,
    M: TryFrom<SerializedBytes>,
{
    let moves = game_move::handlers::get_moves_entries(game_hash.clone())?;
    let game = get_game(game_hash.clone())?;

    let only_moves: Vec<GameMoveEntry> = moves.iter().map(|m| m.1.clone()).collect();

    let state: G = build_game_state(&game, &only_moves)?;

    let serialized_moves = moves
        .into_iter()
        .map(|m| {
            let serialized_move = M::try_from(m.1.clone().game_move)
                .or(Err(WasmError::Guest("Can't convert move".into())))?;
            Ok(MoveInfo {
                move_hash: EntryHashB64::from(m.0),
                move_entry: m.1,
                game_move: serialized_move,
            })
        })
        .collect::<ExternResult<Vec<MoveInfo<M>>>>()?;

    let game_state = GameInfo {
        game_entry: game,
        game_state: state,
        moves: serialized_moves,
    };
    Ok(game_state)
}

pub fn get_game(game_hash: EntryHash) -> ExternResult<GameEntry> {
    let element = get(game_hash, GetOptions::default())?
        .ok_or(WasmError::Guest("There is no game at this hash".into()))?;

    element
        .entry()
        .to_app_option()?
        .ok_or(WasmError::Guest("Couldn't deserialize game entry".into()))
}

pub(crate) fn build_game_state<G, M>(
    game_entry: &GameEntry,
    moves: &Vec<GameMoveEntry>,
) -> ExternResult<G>
where
    G: TurnBasedGame<M>,
    M: TryFrom<SerializedBytes>,
{
    let mut game_state = G::initial(&game_entry.players.clone());

    for (_index, game_move) in moves.iter().enumerate() {
        apply_move(&mut game_state, &game_entry, game_move)?;
    }
    return Ok(game_state);
}

pub(crate) fn apply_move<G, M>(
    game_state: &mut G,
    game_entry: &GameEntry,
    game_move: &GameMoveEntry,
) -> ExternResult<()>
where
    G: TurnBasedGame<M>,
    M: TryFrom<SerializedBytes>,
{
    let move_content = M::try_from(game_move.game_move.clone())
        .or(Err(WasmError::Guest("Coulnt't convert game move".into())))?;

    let author_index = game_entry
        .players
        .iter()
        .enumerate()
        .find_map(|(index, player_address)| {
            if game_move.author_pub_key == player_address.clone() {
                return Some(index);
            } else {
                return None;
            }
        })
        .ok_or(WasmError::Guest(
            "Unreachable: this player is not playing this game".into(),
        ))?;
    game_state.apply_move(&move_content, &game_entry.players, author_index)?;

    Ok(())
}
