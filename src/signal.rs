use crate::{entries::game::GameEntry, prelude::MoveInfo};
use hdk::prelude::*;
use holo_hash::EntryHashB64;

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub enum SignalPayload {
    GameStarted((EntryHashB64, GameEntry)),
    Move(MoveInfo),
}

/**
 * Send a remote signal to all players of the given game
 */
pub fn send_signal_to_players(game: GameEntry, signal: SignalPayload) -> ExternResult<()> {
    let agent_info = agent_info()?;

    let opponents: Vec<AgentPubKey> = game
        .players
        .into_iter()
        .map(|p| AgentPubKey::from(p))
        .filter(|player| player.clone() != agent_info.agent_latest_pubkey.clone())
        .collect();

    remote_signal(ExternIO::encode(signal)?, opponents)?;

    Ok(())
}

/**
 * Receives a new move made by an opponent and emits a signal
 */
#[hdk_extern]
fn recv_remote_signal(signal: ExternIO) -> ExternResult<()> {
    let payload: SignalPayload = signal.decode()?;
    emit_signal(&payload)?;
    Ok(())
}
