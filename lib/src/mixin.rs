use hdk::prelude::*;

pub fn init_turn_based_games() -> ExternResult<InitCallbackResult> {
    // grant unrestricted access to accept_cap_claim so other agents can send us claims
    let mut functions: GrantedFunctions = BTreeSet::new();
    functions.insert((zome_info()?.zome_name, "recv_remote_signal".into()));
    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        // empty access converts to unrestricted
        access: ().into(),
        functions,
    })?;

    Ok(InitCallbackResult::Pass)
}

#[macro_export]
macro_rules! mixin_turn_based_game {
    ( $turn_based_game:ty ) => {
        #[hdk_extern]
        fn get_game_state(game_hash: EntryHashB64) -> ExternResult<$turn_based_game> {
            $crate::get_game_state::<$turn_based_game>(game_hash.into())
        }

        #[hdk_extern]
        fn get_game_moves(game_hash: EntryHashB64) -> ExternResult<Vec<$crate::MoveInfo>> {
            $crate::get_game_moves(game_hash.into())
        }

        #[hdk_extern]
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
        }
    };
}
