use hdk::prelude::*;
use hdk::prelude::holo_hash::AgentPubKeyB64;

/**
 * Game trait that your game struct has to implement
 */
pub trait TurnBasedGame<M>: Sized
where
    M: TryFrom<SerializedBytes>,
{
    // The minimum number of players that must participate for the game to be valid
    // Return None if there is no limit
    fn min_players() -> Option<usize>;

    // The maximum number of players that must participate for the game to be valid
    // Return None if there is no limit
    fn max_players() -> Option<usize>;

    // Constructs the initial state for the game
    fn initial(players: &Vec<AgentPubKeyB64>) -> Self;

    // Applies the move to the game object, transforming it
    // If the move is invalid, it should return an error
    fn apply_move(&mut self, game_move: &M, players: &Vec<AgentPubKeyB64>, author_index: usize) -> ExternResult<()>;

    // Gets the winner for the game
    // Returns None if the game hasn't finished yet
    fn get_winner(&self, players: &Vec<AgentPubKeyB64>) -> Option<AgentPubKeyB64>;
}
