use std::collections::BTreeMap;

use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::{GameEntry, get_game, signal::SignalPayload};

pub fn get_my_current_games() -> ExternResult<BTreeMap<EntryHashB64, GameEntry>> {
    get_current_games_for(agent_info()?.agent_initial_pubkey)
}

pub fn add_current_game(game_hash: EntryHash, players: Vec<AgentPubKeyB64>) -> ExternResult<()> {
    for agent in players {
        create_link(
            AgentPubKey::from(agent).into(),
            game_hash.clone().into(),
            current_games_tag(),
        )?;
    }

    Ok(())
}

pub fn remove_current_game(game_hash: EntryHashB64) -> ExternResult<()> {
    let game = get_game(game_hash.clone())?;
    let my_pub_key = agent_info()?.agent_latest_pubkey;

    remove_my_current_game(game_hash.clone().into())?;

    let players: Vec<AgentPubKeyB64> = game
        .players
        .into_iter()
        .filter(|p| !my_pub_key.eq(&AgentPubKey::from(p.clone())))
        .collect();

    for agent in players {
        let response = call_remote(
            agent.clone().into(),
            zome_info()?.name,
            "notify_remove_my_current_game".into(),
            None,
            game_hash.clone(),
        )?;

        // We notify them via call remote that the game ended so they can remove it
        // If they are not there, we delete the link ourselves for them
        match response {
            ZomeCallResponse::Ok(_) => Ok(()),
            _ => {
                warn!(
                    "Notify agent to remove current game failed, deleting it ourselves {:?}",
                    response
                );
                remove_current_game_for_agent(game_hash.clone().into(), agent.into())
            }
        }?;
    }

    Ok(())
}

pub fn remove_my_current_game(game_hash: EntryHash) -> ExternResult<()> {
    let my_pub_key = agent_info()?.agent_latest_pubkey;

    remove_current_game_for_agent(game_hash.clone(), my_pub_key)?;

    emit_signal(SignalPayload::RemovedCurrentGame {
        game_hash: game_hash.into(),
    })?;

    Ok(())
}

fn remove_current_game_for_agent(
    game_hash: EntryHash,
    agent_pub_key: AgentPubKey,
) -> ExternResult<()> {
    let links_to_current_game = get_current_games_links(agent_pub_key)?
        .into_iter()
        .find(|link| link.target.eq(&game_hash));

    if let Some(link) = links_to_current_game {
        delete_link(link.create_link_hash)?;
    }

    Ok(())
}

fn current_games_tag() -> LinkTag {
    LinkTag::new("current_games")
}

fn get_current_games_for(agent: AgentPubKey) -> ExternResult<BTreeMap<EntryHashB64, GameEntry>> {
    let links = get_current_games_links(agent)?;

    let get_inputs = links
        .into_iter()
        .map(|l| GetInput::new(l.target.into(), GetOptions::default()))
        .collect();

    let elements = HDK.with(|hdk| hdk.borrow().get(get_inputs))?;

    let mut current_games = BTreeMap::new();

    for element in elements.into_iter().filter_map(|m| m) {
        let game_entry: GameEntry = element
            .entry()
            .to_app_option()?
            .ok_or(WasmError::Guest("Could not convert game entry".into()))?;

        let entry_hash = element
            .header()
            .entry_hash()
            .ok_or(WasmError::Guest("Bad create game header".into()))?;
        current_games.insert(entry_hash.clone().into(), game_entry);
    }

    Ok(current_games)
}

fn get_current_games_links(agent: AgentPubKey) -> ExternResult<Vec<Link>> {
    let links = get_links(agent.clone().into(), Some(current_games_tag()))?;
    Ok(links)
}
