# holochain_roles

Generic holochain mixin to include administrator and dynamic roles in any holochain application, using the progenitor pattern.

This mixin is built to target `hc v0.0.46-alpha1`. It also depends on the [holochain_anchors](https://github.com/holochain/holochain_anchors) to be present and configured.

## Design

Here is the design for this mixin: https://hackmd.io/6xfwfSVYSGeZe3vQ_-1cWw?view.

## Documentation

Here you can find the documentation for this mixin: https://docs.rs/holochain_roles.

## Installation

Add the following to your zomes cargo toml.

```
holochain_anchors = { git = "https://github.com/holochain/holochain-anchors" }
holochain_roles = { git = "https://github.com/eyss/holochain-roles" }
```

> We can't publish to crates.io until the holochain_anchors dependency is also published.

## Usage

### Setup

Add the anchor entry definition to your zome.

```rust
 #[entry_def]
fn anchor_def() -> ValidatingEntryType {
    holochain_anchors::anchor_definition()
}
```

Add the roles entry definition to your zome.

```rust
 #[entry_def]
fn roles_def() -> ValidatingEntryType {
    holochain_roles::role_assignment_entry_def()
}
```

### Assign a role

To assign a role, simply call the `assign_role` function:

```rust
#[zome_fn("hc_public")]
fn some_other_public_function(agent_address: Address) {
    let my_role_name = String::from("editor");

    holochain_roles::handlers::assign_role(&my_role_name, &agent_address)?;
    ...
}
```

### Assign an administrator

Only agents that have the administrator role or the progenitor of the DNA can assign or unassign roles.
To assign an administrator role, call the `assign_role` function with the imported administrator role name:

```rust
#[zome_fn("hc_public")]
fn some_other_public_function(agent_address: Address) {
    let my_role_name = String::from(holochain_roles::ADMIN_ROLE_NAME);

    holochain_roles::handlers::assign_role(&my_role_name, &agent_address)?;
    ...
}
```

### Check if user had a certain role in a certain moment in time

**Only use these functions in a validation rule**

To check if a user has a certain role, you have two options:

- Use the validation `validate_required_role` function, which will return and error in case the user did not have the given role at the time they committed the entry:

```rust
validation: | _validation_data: hdk::EntryValidationData<MyEntry>| {
    match _validation_data {
        hdk::EntryValidationData::Create { validation_data } => {
            holochain_roles::validaton::validate_require_role(&validation_data, String::from("editor"))?;

            ...
        }
    }
}
```

- Use the validation `had_agent_role` function:

```rust
validation: | _validation_data: hdk::EntryValidationData<MyEntry>| {
    match _validation_data {
        hdk::EntryValidationData::Create { entry, validation_data } => {
            let agent_address = &validation_data.sources()[0];
            let timestamp = &validation_data.package.chain_header.timestamp();
            let is_agent_permitted_to_create_this_entry = holochain_roles::validaton::had_agent_role(&agent_address, String::from("editor"), timestamp)?;

            if !is_agent_permitted_to_create_this_entry {
                return Err(String::from("Only editors can create a new entry"));
            }
            ...

        }
    }
}
```

### Check if user currently has a certain role

**This should not be used in a validation rule**

To check if a user has a certain role, you can use the `has_agent_role` function:

```rust
#[zome_fn("hc_public")]
fn some_public_function(agent_address: Address) {
   let is_agent_permitted_to_create_this_entry = holochain_roles::validaton::has_agent_role(&agent_address, String::from("editor"))?;
}
```

### Get all role assignments for an agent

To get all role assignments for a certain agent, you can use the validation `get_agent_roles` function:

```rust
#[zome_fn("hc_public")]
fn some_public_function(agent_address: Address) {
    let roles: Vec<String> = holochain_roles::handlers::get_agent_roles(&agent_address)?;
}
```

### Get all agents that have a certain role assigned

To get all role assignments for a certain agent, you can use the validation `get_agents_with_role` function:

```rust
#[zome_fn("hc_public")]
fn some_public_function(role_name: String) {
    let agents: Vec<Address> = holochain_roles::handlers::get_agents_with_role(&role_name)?;
}
```

### Unassign a role

To unassign a role, simply call the `unassign_role` function:

```rust
#[zome_fn("hc_public")]
fn some_other_public_function(agent_address: Address) {
    let my_role_name = String::from("editor");

    holochain_roles::handlers::unassign_role(&my_role_name, &agent_address)?;
    ...
}
```
