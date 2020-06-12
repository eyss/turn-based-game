use crate::game_move;
use crate::game_move::MoveEntry;
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct GameEntry {
    pub players: Vec<Address>,
    pub created_at: u64,
}

impl HolochainEntry for GameEntry {
    fn entry_type() -> String {
        String::from("game_entry")
    }
}

/**
 * Game trait that your game struct has to implement
 */
pub trait TurnBasedGame<M>: Sized
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
    fn apply_move(&mut self, game_move: &M, player_index: usize, author_address: &Address) -> ();

    // Gets the winner for the game
    fn get_winner(
        &self,
        players: &Vec<Address>,
    ) -> Option<Address>;
}

pub fn game_definition<G, M>() -> ValidatingEntryType
where
    G: TurnBasedGame<M>,
    M: TryFrom<JsonString> + Into<JsonString> + Clone,
{
    entry!(
        name: GameEntry::entry_type(),
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
        },
        links: [
            from!(
                "%agent_id",
                link_type: "agent->game",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: |_validation_data: hdk::LinkValidationData| {
                    match _validation_data {
                        hdk::LinkValidationData::LinkAdd { validation_data, link } => {
                            let author = validation_data.package.chain_header.provenances()[0].source();

                            let game: GameEntry = hdk::utils::get_as_type(link.link.target().clone())?;

                            match game.players.contains(&author) {
                                true => Ok(()),
                                false =>  Err(String::from("Only players can link from the agent"))
                            }
                        },
                        _ => Err(String::from("Cannot remove agent->game link"))
                    }
                }
            )
        ]
    )
}

/** Public handlers */

/**
 * Creates the game
 */
pub fn create_game(game: GameEntry) -> ZomeApiResult<Address> {
    let game_address = hdk::commit_entry(&game.clone().entry())?;

    for player in game.players {
        hdk::link_entries(&player, &game_address, "agent->game", "")?;
    }

    Ok(game_address)
}

/**
 * Gets the winner of the game
 */
pub fn get_game_winner<G, M>(game_address: &Address) -> ZomeApiResult<Option<Address>>
where
    G: TurnBasedGame<M>,
    M: TryFrom<JsonString> + Into<JsonString> + Clone,
{
    let game: GameEntry = hdk::utils::get_as_type(game_address.clone())?;

    let moves = game_move::get_moves_entries(&game_address)?;

    compute_winner::<G, M>(game, moves)
}

/**
 * Gets the current state of the game
 */
pub fn get_game_state<G, M>(game_address: &Address) -> ZomeApiResult<G>
where
    G: TurnBasedGame<M>,
    M: TryFrom<JsonString> + Into<JsonString> + Clone,
{
    let game: GameEntry = hdk::utils::get_as_type(game_address.clone())?;

    let moves = game_move::get_moves_entries(&game_address)?;

    let mut game_state = G::initial(&game.players.clone());

    for (index, game_move) in moves.iter().enumerate() {
        let move_content = game_move::parse_move::<M>(game_move.game_move.clone())?;
        game_state.apply_move(
            &move_content,
            index % game.players.len(),
            &game_move.author_address,
        );
    }

    Ok(game_state)
}

/**
 * Get all the games in which the given agent has participated
 */
pub fn get_agent_games(agent_address: &Address) -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &agent_address,
        LinkMatch::Exactly("agent->game"),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}

/** Private helpers */

/**
 * Compute the state for the given game and moves
 */
pub(crate) fn compute_winner<G, M>(
    game: GameEntry,
    moves: Vec<MoveEntry>,
) -> ZomeApiResult<Option<Address>>
where
    G: TurnBasedGame<M>,
    M: TryFrom<JsonString> + Into<JsonString> + Clone,
{
    let mut game_state = G::initial(&game.players.clone());

    for (index, game_move) in moves.iter().enumerate() {
        let move_content = game_move::parse_move::<M>(game_move.game_move.clone())?;
        game_state.apply_move(
            &move_content,
            index % game.players.len(),
            &game_move.author_address,
        );

    }

    Ok(game_state.get_winner(&game.players))
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
