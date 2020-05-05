#![feature(vec_remove_item)]

extern crate holochain_entry_utils;

pub mod game;
pub mod game_move;

pub use game::{Game, create_game, get_game_state, game_definition, get_game_winner};
pub use game_move::{move_definition, create_move, get_last_move, get_game_moves, get_moves_entries};
