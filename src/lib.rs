#![feature(vec_remove_item)]

extern crate holochain_entry_utils;

pub mod game;
pub mod game_move;
pub mod signal;

pub use game::{
    create_game, game_definition, get_agent_games, get_game_state, get_game_winner, TurnBasedGame,
};
pub use game_move::{
    create_move, get_game_moves, get_last_move, get_moves_entries, move_definition,
};
pub use signal::handle_receive_move;
