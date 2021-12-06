# hc_turn_based_game

Generic holochain engine mixin to create turn based games in your holochain apps. These are games with a finite number of players, in which each player takes turns consecutively to play their turn.

This is an update from the previous version of this engine which can be found in https://github.com/holochain-devcamp/generic-game, coded by https://github.com/willemolding.

This mixin is built to target `hdk v0.0.109`.

## Installation

Add the following to your zomes cargo toml.

```
hc_mixin_turn_based_game = { git = "https://github.com/eyss/turn-based-game", branch = "main" }
```

## Usage

We're going to follow all the steps in order to create or turn based game, by using tic-tac-toe as an example (you can find the full hApp example in `example` ).

### 1. Create your game state struct

You need to create a `struct` that represents the state of your game at any point in time. This struct will be serialized into `SerializedBytes` and committed to the DHT, so be careful to optimise its size.

```rust
#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct Piece {
  pub x: usize,
  pub y: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct TicTacToe {
  pub player_1: Vec<Piece>,
  pub player_2: Vec<Piece>,
}
```

### 2. Create your move type

Next, we have to create a move type representing all the possible moves that a player can make when they play their turn. This is normally an enum outlining all the different possible move types, with all the necessary information about the move there. This struct will also be committed to the DHT in a serialized form, so be careful not to load it with too much redundant information.

```rust
#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub enum TicTacToeMove {
  Place(Piece),
  Resign,
}
```

### 3. Implement the Game trait

Next, we need to specify the behaviour of our game. This is done by implementing the `Game` trait:

```rust
impl TurnBasedGame for TicTacToe {
    type GameMove = TicTacToeMove;

    // The minimum number of players that must participate for the game to be valid
    // Return None if there is no limit
    fn min_players() -> Option<usize> {
        Some(2)
    }

    // The maximum number of players that must participate for the game to be valid
    // Return None if there is no limit
    fn max_players() -> Option<usize> {
        Some(2)
    }

    // Constructs the initial state for the game
    fn initial(players: Vec<AgentPubKeyB64>) -> Self {
        ...
    }

    // Applies the move to the game object, transforming it
    // If the move is invalid, it should return an error
    fn apply_move(self, game_move: TicTacToeMove, author: AgentPubKeyB64) -> ExternResult<TicTacToe> {
        ...
    }

    // Returns whether the game has finished or not yet
    fn status(self) -> GameStatus {
        ...
    }
}
```

From now on, when calling most functions in the crate, we'll need to provide the game and move structs as type parameters so that the library can execute its functions.

### 4. Add the game and move entry definitions

```rust
use hc_mixin_turn_based_game::*;

entry_defs![GameMoveEntry::entry_def(), GameEntry::entry_def()];
```

### 5. Call the init function from the zome's `init`

```rust
use hc_mixin_turn_based_game::*;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    init_turn_based_games()
}
```

### 6. Define the mixin

```rust
use hc_mixin_turn_based_game::*;

mixin_turn_based_game!(TicTacToe);
```

This is a macro that will define [all these functions in your zome](/lib/src/mixin.rs). Careful with function name collisions!

## Play a game

### 1. Create a game

To create a game, call the `create_game` function:

```rust
#[hdk_extern]
fn create_game(rival: AgentPubKeyB64) -> ExternResult<EntryHashB64> {
    let hash = hc_mixin_turn_based_game::create_game(vec![
        rival,
        agent_info()?.agent_latest_pubkey.into(),
    ])?;

    Ok(hash)
}
```

The order of the players in the vector will determine the order in which they have to move.

### 2. Get game state

To get the game entry, call `get_game` :

```rust
#[hdk_extern]
fn get_game(game_hash: EntryHashB64) -> ExternResult<GameEntry> {
    hc_mixin_turn_based_game::get_game(game_hash)
}
```

To get the moves that have been done during the game, call `get_game_moves` :

```rust
#[hdk_extern]
fn get_moves(game_hash: EntryHashB64) -> ExternResult<Vec<MoveInfo>> {
    hc_mixin_turn_based_game::get_game_moves(game_hash)
}
```
