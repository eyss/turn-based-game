use crate::{prelude::GameMoveEntry, turn_based_game::TurnBasedGame};

use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use hdk::prelude::*;
use hdk::prelude::holo_hash::{AgentPubKeyB64, EntryHashB64};
use std::{collections::HashMap, convert::TryFrom};

pub mod handlers;

#[hdk_entry(id = "game_entry")]
#[derive(Clone)]
pub struct GameEntry {
    pub players: Vec<AgentPubKeyB64>,
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
}

// IO structs
#[derive(Serialize, Deserialize, Debug)]
pub struct MoveInfo {
    pub move_hash: EntryHashB64,
    pub move_entry: GameMoveEntry,
}

/**
 * Validates the game, returning error if:
 *
 * - There is a repeated player in the game
 * - The number of players is within the bounds defined by the game
 */
pub fn validate_game_entry<G, M>(data: ValidateData) -> ExternResult<ValidateCallbackResult>
where
    G: TurnBasedGame<M>,
    M: TryFrom<SerializedBytes>,
{
    let game: GameEntry = data
        .element
        .entry()
        .to_app_option()?
        .ok_or(WasmError::Guest(
            "Trying to validate an entry that's not a game".into(),
        ))?;

    let mut players_map: HashMap<AgentPubKeyB64, bool> = HashMap::new();

    for player in game.players.iter() {
        if players_map.contains_key(player) {
            return Ok(ValidateCallbackResult::Invalid(format!(
                "Game contains a repeated agent: {}",
                player
            )));
        }
        players_map.insert(player.clone(), true);
    }
    if let Some(min_players) = G::min_players() {
        if game.players.len() < min_players {
            return Ok(ValidateCallbackResult::Invalid(String::from(
                "Bad number of players",
            )));
        }
    }
    if let Some(max_players) = G::max_players() {
        if game.players.len() > max_players {
            return Ok(ValidateCallbackResult::Invalid(String::from(
                "Bad number of players",
            )));
        }
    }

    Ok(ValidateCallbackResult::Valid)
}
