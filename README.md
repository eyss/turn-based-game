# holochain_turn_based_game

Generic holochain engine mixin to create turn based games in your holochain apps. These are games with a finite number of players, in which each player takes turns consecutively to play their turn.

This is an update from the previous version of this engine which can be found in https://github.com/holochain-devcamp/generic-game, coded by https://github.com/willemolding.

This mixin is built to target `hdk v0.0.100` , and published on crates: https://crates.io/crates/holochain_turn_based_game.

## Documentation

Here you can find the documentation for this mixin: https://docs.rs/holochain-turn-based-game.

## Installation

Add the following to your zomes cargo toml.

```
holochain_turn_based_game = "0.2"
```

## Setup

We're going to follow all the steps in order to create or turn based game, by using tic-tac-toe as an example (you can find the full hApp example in `example-dna` ).

### 1. Create your game state struct

You need to create a `struct` that represents the state of your game at any point in time. This struct will not be committed as an entry, so you don't need to optimize for memory or storage as far as the DHT goes.

```rust
#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Piece {
  pub x: usize,
  pub y: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct TicTacToe {
  pub player_1: Vec<Piece>,
  pub player_2: Vec<Piece>,
}
```

### 2. Create your move type

Next, we have to create a move type representing all the possible moves that a player can make when they play their turn. This is normally an enum outlining all the different possible move types, with all the necessary information about the move there. This struct **will** be committed to the DHT in a serialized form, so be careful not to load it with too much redundant information.

```rust
#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub enum TicTacToeMove {
  Place(Piece),
  Resign,
}
```

### 3. Implement the Game trait

Next, we need to specify the behaviour of our game. This is done by implementing the `Game` trait:

```rust

impl Game<TicTacToeMove> for TicTacToe {
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
    fn initial(players: &Vec<AgentPubKey>) -> Self {
        ...
    }

    // Applies the move to the game object, transforming it
    // If the move is invalid, it should return an error
    fn apply_move(&mut self, game_move: &M, players: &Vec<AgentPubKey>, author_index: usize) -> ExternResult<()> {
        ...
    }

    // Gets the winner for the game
    fn get_winner(
      &self,
      players: &Vec<Address>,
    ) -> Option<AgentPubKey> {
        ...
    }
}
```

From now on, when calling most functions in the crate, we'll need to provide the game and move structs as type parameters so that the library can execute its functions.

### 4. Add the game and move entry definitions

```rust
use holochain_turn_based_game::prelude::*;

entry_defs![GameMoveEntry::entry_def(), GameEntry::entry_def()];
```

### 5. Call the init function from the zome's `init`

```rust
use holochain_turn_based_game::prelude::*;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    init_turn_based_games()
}
```

## Play a game

### 1. Create a game

To create a game, call the `create_game` function:

```rust
#[hdk_extern]
fn create_game(rival: AgentPubKeyB64) -> ExternResult<EntryHashB64> {
    let hash = holochain_turn_based_game::prelude::create_game(vec![
        rival.into(),
        agent_info()?.agent_latest_pubkey,
    ])?;

    Ok(hash.into())
}
```

The order of the players in the vector will determine the order in which they have to move.

### 2. Make a move

To create a move, call the `create_move` function:

```rust
#[derive(Serialize, Deserialize, Debug)]
struct PlacePieceInput {
    game_hash: EntryHashB64,
    previous_move_hash: Option<EntryHashB64>,
    x: usize,
    y: usize,
}
#[hdk_extern]
fn place_piece(
    PlacePieceInput {
        game_hash,
        previous_move_hash,
        x,
        y,
    }: PlacePieceInput,
) -> ExternResult<EntryHashB64> {
    let game_move = TicTacToeMove::Place(Piece { x, y });
    let move_hash = holochain_turn_based_game::prelude::create_move(
        game_hash.into(),
        previous_move_hash.map(|hash| hash.into()),
        game_move,
    )?;
    Ok(move_hash.into())
}
```

### 3. Get game state

To get the current game information and state, call `get_game_state` :

```rust
#[hdk_extern]
fn get_game_info(game_hash: EntryHashB64) -> ExternResult<GameInfo<TicTacToe, TicTacTeoMove>> {
    holochain_turn_based_game::prelude::get_game_info::<TicTacToe, TicTacToeMove>(game_hash.into())
}
```

To get the moves that have been done during the game, call `get_game_moves` :

```rust
#[zome_fn("hc_public")]
fn get_moves(game_hash: EntryHashB64) -> ExternResult<Vec<TicTacToeMove>> {
    holochain_turn_based_game::prelude::get_game_moves::<TicTacToe, TicTacToeMove>(game_hash.into())
}
```

And to get the winner of the game, call `get_game_winner` :

```rust
#[hdk_extern]
fn get_winner(game_hash: EntryHashB64) -> ExternResult<Option<AgentPubKeyB64>> {
    let winner = holochain_turn_based_game::prelude::get_game_winner::<TicTacToe, TicTacToeMove>(
        game_hash.into(),
    )?;

    Ok(winner.map(|w| w.into()))
}
```
