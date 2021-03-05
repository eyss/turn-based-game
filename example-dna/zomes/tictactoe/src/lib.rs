use hdk::prelude::*;
use holochain_turn_based_game;
use holochain_turn_based_game::game::GameEntry;

mod tictactoe;

use tictactoe::{Piece, TicTacToe, TicTacToeMove};

entry_defs![];

#[hdk_extern]
fn create_game(rival: Address) -> ExternResult<Address> {
    let game = GameEntry {
        players: vec![rival, hdk::AGENT_ADDRESS.clone()],
        created_at: timestamp,
    };

    holochain_turn_based_game::create_game(game)
}

#[zome_fn("hc_public")]
fn place_piece(game_address: Address, x: usize, y: usize) -> ZomeApiResult<Address> {
    let game_move = TicTacToeMove::Place(Piece { x, y });
    holochain_turn_based_game::create_move(&game_address, game_move)
}

#[zome_fn("hc_public")]
fn get_winner(game_address: Address) -> ZomeApiResult<Option<Address>> {
    holochain_turn_based_game::get_game_winner::<TicTacToe, TicTacToeMove>(&game_address)
}

#[zome_fn("hc_public")]
fn get_agent_games(agent_address: Address) -> ZomeApiResult<Vec<Address>> {
    holochain_turn_based_game::get_agent_games(&agent_address)
}

#[zome_fn("hc_public")]
fn get_game_state(game_address: Address) -> ZomeApiResult<TicTacToe> {
    holochain_turn_based_game::get_game_state::<TicTacToe, TicTacToeMove>(&game_address)
}

#[receive]
fn receive(sender_address: Address, message: String) -> String {
    let result = holochain_turn_based_game::handle_receive_move(sender_address, message);

    JsonString::from(result).to_string()
}
