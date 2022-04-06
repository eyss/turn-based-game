//use hdk::prelude::*;

mod current_games;
mod game;
mod game_move;
mod signal;
mod turn_based_game;

mod mixin;

/*#[hdk_extern]
pub fn validate(op:Op) -> ExternResult<ValidateCallbackResult>{
    match op{
     Ok(ValidateCallbackResult::Valid)
    }
}*/

pub use current_games::{get_my_current_games, remove_my_current_game, remove_current_game};
pub use game::{
    create_game, get_game, get_game_state, GameEntry //validate_game_entry, GameEntry,
};
pub use game_move::{
    create_move, get_game_moves, GameMoveEntry, MoveInfo, //validate_game_move_entry, GameMoveEntry, MoveInfo,
};
pub use mixin::*;
pub use turn_based_game::*;



