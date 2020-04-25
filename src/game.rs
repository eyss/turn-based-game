use hdk::prelude::*;
use std::collections::HashMap;
use holochain_entry_utils::HolochainEntry;

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct GameEntry {
  pub players: Vec<Address>,
  pub created_at: u32,
}

impl HolochainEntry for GameEntry {
  fn entry_type() -> String {
    String::from("game_entry")
  }
}

pub trait Game {
  // Validates that the entry for the given game
  // By default only looks at the number of players for the game
  fn validate_entry(game: &GameEntry) -> ZomeApiResult<()> {
    if let Some(min_players) = Self::min_players() {
      if game.players.len() < min_players {
        return Err(ZomeApiError::from(String::from("Bad number of players")));
      }
    }
    if let Some(max_players) = Self::max_players() {
      if game.players.len() > max_players {
        return Err(ZomeApiError::from(String::from("Bad number of players")));
      }
    }

    Ok(())
  }

  // The minimum number of players that must participate for the game to be valid
  // Return None if there is no limit
  fn min_players() -> Option<usize>;

  // The maximum number of players that must participate for the game to be valid
  // Return None if there is no limit
  fn max_players() -> Option<usize>;

  // Constructs the initial state for the game
  fn initial() -> Self;
}

pub fn definition<G>() -> ValidatingEntryType
where
  G: Game,
{
  entry!(
      name: "game",
      description: "Represents an occurence of a game between several agents",
      sharing: Sharing::Public,
      validation_package: || {
          hdk::ValidationPackageDefinition::Entry
      },

      validation: | validation_data: hdk::EntryValidationData<GameEntry>| {
          match validation_data {
              EntryValidationData::Create{ entry, .. } => {
                  validate_game_entry(entry.clone())?;

                  G::validate_entry(&entry)?;

                  Ok(())
              },
              _ => {
                  Err("Cannot modify or delete a game".into())
              }
          }
      }
  )
}

/**
 * Validates the game, returning error if:
 *
 * - There is a repeated player in the game
 */
pub fn validate_game_entry(game: GameEntry) -> ZomeApiResult<()> {
  let mut players_map: HashMap<Address, bool> = HashMap::new();

  for player in game.players.iter() {
    if players_map.contains_key(player) {
      return Err(ZomeApiError::from(format!(
        "Game contains a repeated agent: {}",
        player
      )));
    }
    players_map.insert(player.clone(), true);
  }

  Ok(())
}
