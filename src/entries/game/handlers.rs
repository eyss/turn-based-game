use chrono::{DateTime, NaiveDateTime, Utc};
use hdk::prelude::*;

use crate::{entries::game_move, turn_based_game::TurnBasedGame};

use super::GameEntry;

/** Public handlers */

/**
 * Creates the game
 */
pub fn create_game(players: Vec<AgentPubKey>) -> ExternResult<EntryHash> {
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

    Ok(game_hash)
}

/**
 * Gets the winner of the game
 */
pub fn get_game_winner<G, M>(game_hash: EntryHash) -> ExternResult<Option<AgentPubKey>>
where
    G: TurnBasedGame<M>,
    M: TryFrom<SerializedBytes>,
{
    let game = get_game(game_hash)?;
    let game_state = get_game_state::<G, M>(game_hash)?;

    Ok(game_state.get_winner(&game.players))
}

/**
 * Gets the current state of the game
 */
pub fn get_game_state<G, M>(game_hash: EntryHash) -> ExternResult<G>
where
    G: TurnBasedGame<M>,
    M: TryFrom<SerializedBytes>,
{
    let moves = game_move::handlers::get_moves_entries(game_hash)?;
    let game = get_game(game_hash)?;

    let mut game_state = G::initial(&game.players.clone());

    for (index, game_move) in moves.iter().enumerate() {
        let move_content = M::try_from(game_move.game_move)
            .or(Err(WasmError::Guest("Coulnt't convert game move".into())))?;
        game_state.apply_move(&move_content, &game_move.author_pub_key)?;
    }

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
