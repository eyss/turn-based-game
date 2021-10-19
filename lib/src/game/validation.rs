use std::collections::HashMap;

use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use super::GameEntry;
use crate::TurnBasedGame;

/**
 * Validates the game, returning error if:
 *
 * - There is a repeated player in the game
 * - The number of players is within the bounds defined by the game
 */
pub fn validate_game_entry<G: TurnBasedGame>(
    data: ValidateData,
) -> ExternResult<ValidateCallbackResult> {
    let maybe_game: Option<GameEntry> = data.element.entry().to_app_option()?;

    if let Some(game) = maybe_game {
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
    } else {
        return match data.element.header().entry_hash() {
            None => Ok(ValidateCallbackResult::Invalid(
                "Bad create game element: no entry inside".into(),
            )),
            Some(game_entry_hash) => Ok(ValidateCallbackResult::UnresolvedDependencies(vec![
                game_entry_hash.clone().into(),
            ])),
        };
    }
    Ok(ValidateCallbackResult::Valid)
}
