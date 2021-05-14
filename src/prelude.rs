pub use crate::entries::game::{
    handlers::{create_game, get_game, get_game_winner, get_game_state},
    validate_game_entry, GameEntry, MoveInfo,
};
pub use crate::entries::game_move::{
    handlers::{create_move, get_game_moves},
    validate_game_move_entry, GameMoveEntry,
};
pub use crate::turn_based_game::TurnBasedGame;

use hdk::prelude::*;

pub fn init_turn_based_games(_: ()) -> ExternResult<InitCallbackResult> {
    // grant unrestricted access to accept_cap_claim so other agents can send us claims
    let mut functions: GrantedFunctions = HashSet::new();
    functions.insert((zome_info()?.zome_name, "recv_remote_signal".into()));
    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        // empty access converts to unrestricted
        access: ().into(),
        functions,
    })?;

    Ok(InitCallbackResult::Pass)
}
