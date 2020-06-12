use crate::{game::GameEntry, game_move::MoveEntry};
use hdk::holochain_core_types::time::Timeout;
use hdk::prelude::*;
use hdk::AGENT_ADDRESS;
use std::convert::TryInto;

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub struct MessageBody {
    game_move: MoveEntry,
}

/**
 * Sends the newly created move to all opponents of the game
 */
pub fn send_move_signal(game_address: &Address, game_move: &MoveEntry) -> ZomeApiResult<()> {
    let game: GameEntry = hdk::utils::get_as_type(game_address.clone())?;

    let opponents: Vec<Address> = game
        .players
        .into_iter()
        .filter(|player| player.clone() != AGENT_ADDRESS.clone())
        .collect();

    let message = MessageBody {
        game_move: game_move.clone(),
    };

    for opponent in opponents {
        hdk::send(
            opponent,
            JsonString::from(message.clone()).to_string(),
            Timeout::new(2000),
        )?;
    }

    Ok(())
}

/**
 * Receives a new move made by an opponent and emits a signal
 */
pub fn handle_receive_move(sender_address: Address, message: String) -> ZomeApiResult<()> {
    let success: Result<MessageBody, _> = JsonString::from_json(&message).try_into();

    let message = success?;

    if message.game_move.author_address != sender_address {
        return Err(ZomeApiError::from(String::from("The author of the move is not the sender of the message")));
    }

    hdk::emit_signal("opponent-moved", JsonString::from(message.game_move))
}
