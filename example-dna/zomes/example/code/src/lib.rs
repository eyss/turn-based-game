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
use holochain_roles;

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
    fn custom_entry() -> ValidatingEntryType {
        entry!(
            name: "test",
            description: "a test entry to validate that roles are working",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<String>| {
                match _validation_data {
                    hdk::EntryValidationData::Create { validation_data, .. } => {
                        holochain_roles::validation::validate_required_role(&validation_data, &String::from("editor"))?;

                        Ok(())
                    },
                    _ => Err(String::from("Cannot modify roles"))
                }
            }
        )
    }
    
    #[zome_fn("hc_public")]
    fn create_test_entry(test: String) -> ZomeApiResult<Address> {
        let entry = Entry::App("test".into(), JsonString::from_json(test.as_str()));
        hdk::commit_entry(&entry)
    }

    #[entry_def]
    fn role_entry_def() -> ValidatingEntryType {
        holochain_roles::role_assignment_entry_def()
    }

    #[entry_def]
    fn anchors_entry_def() -> ValidatingEntryType {
        holochain_anchors::anchor_definition()
    }

    #[zome_fn("hc_public")]
    fn assign_role(role_name: String, agent_address: Address) -> ZomeApiResult<()> {
        holochain_roles::handlers::assign_role(&role_name, &agent_address)
    }

    #[zome_fn("hc_public")]
    fn unassign_role(role_name: String, agent_address: Address) -> ZomeApiResult<()> {
        holochain_roles::handlers::unassign_role(&role_name, &agent_address)
    }

    #[zome_fn("hc_public")]
    fn get_agents_with_role(role_name: String) -> ZomeApiResult<Vec<Address>> {
        holochain_roles::handlers::get_agents_with_role(&role_name)
    }

    #[zome_fn("hc_public")]
    fn get_agent_roles(agent_address: Address) -> ZomeApiResult<Vec<String>> {
        holochain_roles::handlers::get_agent_roles(&agent_address)
    }
}
