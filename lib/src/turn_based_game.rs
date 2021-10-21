use hdk::prelude::holo_hash::AgentPubKeyB64;
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameStatus {
    Finished,
    Ongoing,
}

/**
 * Game trait that your game struct has to implement
 */
pub trait TurnBasedGame: TryFrom<SerializedBytes> + TryInto<SerializedBytes> {
    type GameMove: TryFrom<SerializedBytes> + TryInto<SerializedBytes> + Clone;

    // The minimum number of players that must participate for the game to be valid
    // Return None if there is no limit
    fn min_players() -> Option<usize>;

    // The maximum number of players that must participate for the game to be valid
    // Return None if there is no limit
    fn max_players() -> Option<usize>;

    // Constructs the initial state for the game
    fn initial(players: Vec<AgentPubKeyB64>) -> Self;

    // Applies the move to the game object, transforming it
    // If the move is invalid, it should return an error
    fn apply_move(
        &mut self,
        game_move: Self::GameMove,
        author: AgentPubKeyB64,
        players: Vec<AgentPubKeyB64>,
    ) -> ExternResult<()>;

    // Gets the outcome for the game
    fn status(&self, players: Vec<AgentPubKeyB64>) -> GameStatus;
}
