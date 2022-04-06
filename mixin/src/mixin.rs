use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::TurnBasedGame;

pub fn init_turn_based_games() -> ExternResult<InitCallbackResult> {
    // grant unrestricted access to accept_cap_claim so other agents can send us claims
    let mut functions: GrantedFunctions = BTreeSet::new();
    functions.insert((zome_info()?.name, "recv_remote_signal".into()));
    functions.insert((zome_info()?.name, "notify_remove_my_current_game".into()));
    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        // empty access converts to unrestricted
        access: ().into(),
        functions,
    })?;

    Ok(InitCallbackResult::Pass)
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct MakeMoveInput<G: TurnBasedGame> {
    pub game_hash: EntryHashB64,
    pub previous_move_hash: Option<HeaderHashB64>,
    pub game_move: G::GameMove,
}

#[macro_export]
macro_rules! mixin_turn_based_game {
    ( $turn_based_game:ty ) => {
        #[hdk_extern]
        fn get_my_current_games(
            _: (),
        ) -> ExternResult<std::collections::BTreeMap<EntryHashB64, $crate::GameEntry>> {
            $crate::get_my_current_games()
        }

        #[hdk_extern]
        fn make_move(
            input: $crate::MakeMoveInput<$turn_based_game>,
        ) -> ExternResult<hdk::prelude::holo_hash::HeaderHashB64> {
            $crate::create_move::<$turn_based_game>(
                input.game_hash,
                input.previous_move_hash,
                input.game_move,
            )
        }

        #[hdk_extern]
        fn get_game_moves(game_hash: EntryHashB64) -> ExternResult<Vec<$crate::MoveInfo>> {
            $crate::get_game_moves(game_hash.into())
        }

        #[hdk_extern]
        fn notify_remove_my_current_game(game_hash: EntryHashB64) -> ExternResult<()> {
            $crate::remove_my_current_game(game_hash.into())
        }

        #[hdk_extern]
        fn get_game(game_hash: EntryHashB64) -> ExternResult<$crate::GameEntry> {
            $crate::get_game(game_hash)
        }

       /*  #[hdk_extern]
        fn validate_create_entry_game_entry(
            validate_data: ValidateData,
        ) -> ExternResult<ValidateCallbackResult> {
            $crate::validate_game_entry::<$turn_based_game>(validate_data)
        }

        #[hdk_extern]
        fn validate_create_entry_game_move_entry(
            validate_data: ValidateData,
        ) -> ExternResult<ValidateCallbackResult> {
            $crate::validate_game_move_entry::<$turn_based_game>(validate_data)
        }

        #[hdk_extern]
        fn validate_update_entry_game_move_entry(
            validate_data: ValidateData,
        ) -> ExternResult<ValidateCallbackResult> {
            Ok(ValidateCallbackResult::Invalid(
                "Cannot update game moves".into(),
            ))
        }

        #[hdk_extern]
        fn validate_delete_entry_game_entry(
            validate_data: ValidateData,
        ) -> ExternResult<ValidateCallbackResult> {
            Ok(ValidateCallbackResult::Invalid(
                "Cannot delete games".into(),
            ))
        }

        #[hdk_extern]
        fn validate_update_entry_game_entry(
            validate_data: ValidateData,
        ) -> ExternResult<ValidateCallbackResult> {
            Ok(ValidateCallbackResult::Invalid(
                "Cannot update games".into(),
            ))
        }*/
    };
}
