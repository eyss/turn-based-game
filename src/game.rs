use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;
use std::collections::HashMap;
use std::convert::TryFrom;

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

pub struct AuthoredMove<M>
where
  M: TryFrom<JsonString> + Into<JsonString> + Clone,
{
  pub game_move: M,
  pub author_address: Address,
}

pub trait Game<M>: Sized
where
  M: TryFrom<JsonString> + Into<JsonString> + Clone,
{
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
  fn initial(players: &Vec<Address>) -> Self;

  // Returns whether the given movement is valid given the current game state
  fn is_valid(self, game_move: M) -> Result<(), String>;

  // Applies the move to the game object, transforming it
  fn apply_move(&mut self, author_address: &Address, game_move: &M) -> ();

  // Gets the winner for the game
  fn get_winner(&self, moves: &Vec<AuthoredMove<M>>) -> Option<Address>;
}

pub fn definition<G, M>() -> ValidatingEntryType
where
  G: Game<M>,
  M: TryFrom<JsonString> + Into<JsonString> + Clone,
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
fn validate_game_entry(game: GameEntry) -> ZomeApiResult<()> {
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

/**
 * Creates the game
 */
pub fn create_game(game: GameEntry) -> ZomeApiResult<Address> {
  hdk::commit_entry(&game.entry())
}
