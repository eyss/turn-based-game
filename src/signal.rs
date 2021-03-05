use hdk::prelude::*;

use crate::entries::{game::GameEntry, game_move::MoveEntry};

/**
 * Sends the newly created move to all opponents of the game
 */
pub fn send_move_signal(game_hash: EntryHash, game_move: MoveEntry) -> ExternResult<()> {
    let element = get(game_hash, GetOptions::default())?
        .ok_or(WasmError::Guest("Could not get game entry".into()))?;

    let game: GameEntry = element
        .entry()
        .to_app_option()?
        .ok_or(WasmError::Guest("Failed to convert game entry".into()))?;

    let agent_info = agent_info()?;

    let opponents: Vec<AgentPubKey> = game
        .players
        .into_iter()
        .filter(|player| player.clone() != agent_info.agent_latest_pubkey.clone())
        .collect();

    remote_signal(game_move, opponents)?;

    Ok(())
}

/**
 * Receives a new move made by an opponent and emits a signal
 */
#[hdk_extern]
fn recv_remote_signal(signal: SerializedBytes) -> ExternResult<()> {
    emit_signal(&signal)?;
    Ok(())
}
