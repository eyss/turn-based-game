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
use holochain_turn_based_game::GameEntry;

mod tictactoe;

use tictactoe::{TicTacToe, TicTacToeMove};

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
        holochain_turn_based_game::game::definition::<TicTacToe>()
    }

    #[entry_def]
    fn move_entry() -> ValidatingEntryType {
        holochain_turn_based_game::game_move::definition::<TicTacToe, TicTacToeMove>()
    }

    #[zome_fn("hc_public")]
    fn create_game(game: GameEntry) -> ZomeApiResult<Vec<Address>> {
        holochain_turn_based_game::game::create_game(game)
    }
}
