use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;
use std::convert::TryFrom;

#[hdk_entry(id = "game_entry")]
#[derive(Clone)]
pub struct GameEntry {
    pub players: Vec<AgentPubKeyB64>,
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
}
