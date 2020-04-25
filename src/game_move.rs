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

pub fn definition<G, M>() -> ValidatingEntryType
where
    G: Game<M>,
    M: TryFrom<JsonString> + Into<JsonString> + Clone,
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

                    validate_move::<G, M>(entry)?;

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

/** Public handlers */

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
 * Creates the next move for the given game, linking the game to the move
 */
pub fn create_move<M>(
    game_address: &Address,
    game_move: M,
    current_move: &Option<Address>,
) -> ZomeApiResult<Address>
where
    M: TryFrom<JsonString> + Into<JsonString> + Clone,
{
    let move_json = game_move.into();

    let game_move = MoveEntry {
        game_address: game_address.clone(),
        author_address: hdk::AGENT_ADDRESS.clone(),
        game_move: move_json,
        previous_move_address: current_move.clone(),
    };

    let move_address = hdk::commit_entry(&game_move.entry())?;
    hdk::link_entries(&game_address, &move_address, "game->move", "")?;

    Ok(move_address)
}

/**
 * Get all the moves for the given game
 */
pub fn get_moves<M>(game_address: &Address) -> ZomeApiResult<Vec<M>>
where
    M: TryFrom<JsonString> + Into<JsonString> + Clone,
{
    let moves = get_game_moves(&game_address)?;

    moves
        .iter()
        .map(|m| parse_move(m.game_move.clone()))
        .collect()
}

/**
 * Returns all the moves for the given game
 */
pub fn get_game_moves(game_address: &Address) -> ZomeApiResult<Vec<MoveEntry>> {
    hdk::utils::get_links_and_load_type(
        &game_address,
        LinkMatch::Exactly("game->move"),
        LinkMatch::Any,
    )
}

/** Private helpers */

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

/**
 * Validates the move, getting the game
 */
fn validate_move<G, M>(next_move: MoveEntry) -> ZomeApiResult<()>
where
    G: Game<M>,
    M: TryFrom<JsonString> + Into<JsonString> + Clone,
{
    let game: GameEntry = hdk::utils::get_as_type(next_move.game_address.clone())?;

    if !game.players.contains(&next_move.author_address) {
        return Err(ZomeApiError::from(String::from(
            "The author of the move is not playing the game",
        )));
    }

    let mut moves: Vec<MoveEntry> = get_game_moves(&next_move.game_address)?;

    let ordered_moves = get_ordered_moves(&mut moves)?;

    let maybe_last_move = ordered_moves.last();

    validate_it_is_authors_turn(&next_move.author_address, &maybe_last_move, &game.players)?;

    let mut game_state = G::initial();
    let mut parsed_moves: Vec<M> = Vec::new();

    for game_move in ordered_moves {
        let move_content = parse_move::<M>(game_move.game_move)?;
        game_state.execute(move_content.clone());
        parsed_moves.push(move_content);
    }

    // Get the winner
    let winner = game_state.get_winner(&parsed_moves);

    if let Some(winner_address) = winner {
        return Err(ZomeApiError::from(format!(
            "Game is already finished: {} is the winner",
            winner_address
        )));
    }

    let move_content = parse_move::<M>(next_move.game_move)?;

    match game_state.is_valid(move_content) {
        true => Ok(()),
        false => Err(ZomeApiError::from(String::from("Move is not valid"))),
    }
}

/**
 * Convert the serialized move into the struct
 */
fn parse_move<M>(move_content: JsonString) -> ZomeApiResult<M>
where
    M: TryFrom<JsonString> + Into<JsonString> + Clone,
{
    match M::try_from(move_content) {
        Ok(game_move) => Ok(game_move),
        Err(_) => {
            return Err(ZomeApiError::from(String::from("Bad move")));
        }
    }
}
