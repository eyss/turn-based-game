use hdk::prelude::holo_hash::*;
use hdk::prelude::*;
use std::convert::TryFrom;

#[hdk_entry(id = "game_move_entry")]
#[derive(Clone)]
pub struct GameMoveEntry {
    pub game_hash: EntryHashB64,
    pub author_pub_key: AgentPubKeyB64,
    pub game_move: SerializedBytes,
    pub previous_move_hash: Option<HeaderHashB64>,
}

// IO structs
#[derive(Serialize, Deserialize, Debug)]
pub struct MoveInfo {
    pub header_hash: HeaderHashB64,
    pub game_move_entry: GameMoveEntry,
}
