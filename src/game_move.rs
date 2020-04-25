use crate::game::Game;
use crate::game::GameEntry;
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;
use std::convert::TryFrom;

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub struct MoveEntry {
    pub game_address: Address,
    pub author_address: Address,
    pub game_move: JsonString,
    pub previous_move_address: Option<Address>,
}

impl HolochainEntry for MoveEntry {
    fn entry_type() -> String {
        String::from("move_entry")
    }
}

pub trait Move<G>: TryFrom<JsonString> + Into<JsonString>
where
    G: Game,
{
    // Returns whether the given movement is valid given the current game state
    fn is_valid(self, game: G) -> bool;

    // Applies the move to the game object, transforming it
    fn execute(self, game: G) -> G;
}

pub fn definition<M, G>() -> ValidatingEntryType
where
    M: Move<G>,
    G: Game,
{
    entry!(
        name: "move",
        description: "A move by an agent in a game",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | validation_data: hdk::EntryValidationData<MoveEntry>| {
            match validation_data {
                EntryValidationData::Create { entry, validation_data } => {
                    if validation_data.package.chain_header.provenances()[0].source() != entry.author_address.clone() {
                        return Err(String::from("Move has to be signed by its author"));
                    }

                    validate_move::<M, G>(entry)?;

                    Ok(())
                },
                _ => {
                    Err("Cannot modify or delete a move".into())
                }
            }
        },

        links: [
          from!(
                "game",
                link_type: "game->move",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}

/**
 * Validates the move, getting the game
 */
pub fn validate_move<M, G>(next_move: MoveEntry) -> ZomeApiResult<()>
where
    M: Move<G>,
    G: Game,
{
    let game: GameEntry = hdk::utils::get_as_type(next_move.game_address.clone())?;

    if !game.players.contains(&next_move.author_address) {
        return Err(ZomeApiError::from(String::from(
            "The author of the move is not playing the game",
        )));
    }

    let mut moves: Vec<MoveEntry> = hdk::utils::get_links_and_load_type(
        &next_move.game_address,
        LinkMatch::Exactly("game->move"),
        LinkMatch::Any,
    )?;

    let ordered_moves = get_ordered_moves(&mut moves)?;

    let maybe_last_move = ordered_moves.last();

    validate_it_is_authors_turn(&next_move.author_address, &maybe_last_move, &game.players)?;

    let mut game_state = G::initial();

    for game_move in ordered_moves {
        let move_content = parse_move::<M, G>(game_move.game_move)?;
        game_state = move_content.execute(game_state);
    }

    let move_content = parse_move::<M, G>(next_move.game_move)?;

    match move_content.is_valid(game_state) {
        true => Ok(()),
        false => Err(ZomeApiError::from(String::from("Move is not valid"))),
    }
}

pub fn parse_move<M, G>(move_content: JsonString) -> ZomeApiResult<M>
where
    M: Move<G>,
    G: Game,
{
    match M::try_from(move_content) {
        Ok(game_move) => Ok(game_move),
        Err(_) => {
            return Err(ZomeApiError::from(String::from("Bad move")));
        }
    }
}

/**
 * Returns the moves ordered following the previous_move_address
 *
 * Returns error if in any case the chain of moves is not valid
 */
pub fn get_ordered_moves(moves: &mut Vec<MoveEntry>) -> ZomeApiResult<Vec<MoveEntry>> {
    let mut ordered_moves: Vec<MoveEntry> = Vec::new();

    // Find first move
    let mut current_move = find_next_move(&None, moves)?;
    ordered_moves.push(current_move.clone());

    // Find next move until the vector is empty
    while moves.len() > 0 {
        current_move = find_next_move(&current_move.previous_move_address, moves)?;
        ordered_moves.push(current_move.clone());
    }

    Ok(ordered_moves)
}

/**
 * Finds the next move for the given previous move and game,
 * which is the one where previous_move_address equals the given previous_move_address
 * It also removes that move from the vector
 *
 * Returns error if there is not only one next move
 */
fn find_next_move(
    previous_move_address: &Option<Address>,
    moves: &mut Vec<MoveEntry>,
) -> ZomeApiResult<MoveEntry> {
    let mut move_index: Option<usize> = None;

    for (index, next_move) in moves.iter().enumerate() {
        if next_move.previous_move_address == previous_move_address.clone() {
            if let Some(_) = move_index {
                return Err(ZomeApiError::from(String::from(
                    "Bad number of first moves",
                )));
            }

            move_index = Some(index);
        }
    }
    match move_index {
        Some(index) => Ok(moves.remove(index)),
        None => Err(ZomeApiError::from(String::from(
            "Bad number of first moves",
        ))),
    }
}

/**
 * Validate that it's the turn of the author of the move
 */
fn validate_it_is_authors_turn(
    author_address: &Address,
    maybe_last_move: &Option<&MoveEntry>,
    players: &Vec<Address>,
) -> ZomeApiResult<()> {
    let maybe_last_player_index = match maybe_last_move {
        Some(last_move) => players
            .iter()
            .position(|p| p.clone() == last_move.author_address),
        None => None,
    };

    // Get the index of the player whose turn it is
    let player_index = match maybe_last_player_index {
        Some(last_player_index) => {
            let new_index = last_player_index + 1;

            match new_index >= players.len() {
                true => 0,
                false => new_index,
            }
        }
        None => 0,
    };

    if players[player_index] != author_address.clone() {
        return Err(ZomeApiError::from(String::from(
            "It's not the turn of the author of the move",
        )));
    }

    Ok(())
}
