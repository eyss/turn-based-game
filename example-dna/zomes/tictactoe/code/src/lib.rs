#![feature(vec_remove_item)]
#![feature(proc_macro_hygiene)]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_json_derive;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult};

use hdk::holochain_persistence_api::cas::content::Address;

use hdk::prelude::*;
use hdk_proc_macros::zome;
use holochain_turn_based_game;
use holochain_turn_based_game::game::GameEntry;

mod tictactoe;

use tictactoe::{Piece, TicTacToe, TicTacToeMove};

#[zome]
mod my_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn game_entry() -> ValidatingEntryType {
        holochain_turn_based_game::game_definition::<TicTacToe, TicTacToeMove>()
    }

    #[entry_def]
    fn move_entry() -> ValidatingEntryType {
        holochain_turn_based_game::move_definition::<TicTacToe, TicTacToeMove>()
    }

    #[zome_fn("hc_public")]
    fn create_game(rival: Address, timestamp: u32) -> ZomeApiResult<Address> {
        let game = GameEntry {
            players: vec![rival, hdk::AGENT_ADDRESS.clone()],
            created_at: timestamp,
        };

        holochain_turn_based_game::create_game(game)
    }

    #[zome_fn("hc_public")]
    fn place_piece(game_address: Address, x: usize, y: usize) -> ZomeApiResult<Address> {
        let game_move = TicTacToeMove::Place(Piece { x, y });
        let previous_move = holochain_turn_based_game::get_last_move(&game_address)?;
        holochain_turn_based_game::create_move(&game_address, game_move, &previous_move)
    }

    #[zome_fn("hc_public")]
    fn get_winner(game_address: Address) -> ZomeApiResult<Option<Address>> {
        holochain_turn_based_game::get_game_winner::<TicTacToe, TicTacToeMove>(&game_address)
    }
}
