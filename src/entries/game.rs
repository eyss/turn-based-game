use crate::turn_based_game::TurnBasedGame;

use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use hdk::prelude::*;
use std::{collections::HashMap, convert::TryFrom};

pub mod handlers;

#[hdk_entry(id = "game_entry")]
pub struct GameEntry {
    pub players: Vec<AgentPubKey>,
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
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

    let mut players_map: HashMap<AgentPubKey, bool> = HashMap::new();

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