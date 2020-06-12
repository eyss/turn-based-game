# holochain_turn_based_game

Generic holochain engine mixin to create turn based games in your holochain apps. These are games with a finite number of players, in which each player takes turns consecutively to play their turn.

This is an update from the previous version of this engine which can be found in https://github.com/holochain-devcamp/generic-game, coded by https://github.com/willemolding.

This mixin is built to target `hc v0.0.47-alpha1` , and published on crates: https://crates.io/crates/holochain_turn_based_game.

## Documentation

Here you can find the documentation for this mixin: https://docs.rs/holochain-turn-based-game.

## Installation

Add the following to your zomes cargo toml.

``` 
holochain_roles = "0.1"
```

## Setup

We're going to follow all the steps in order to create or turn based game, by using tic-tac-toe as an example (you can find the full hApp example in `example-dna` ).

### 1. Create your game state struct

You need to create a `struct` that represents the state of your game at any point in time. This struct will not be committed as an entry, so you don't need to optimize for memory or storage as far as the DHT goes.

``` rust
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

``` rust
#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub enum TicTacToeMove {
  Place(Piece),
  Resign,
}
```

### 3. Implement the Game trait

Next, we need to specify the behaviour of our game. This is done by implementing the `Game` trait:

``` rust

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
    fn initial(players: &Vec<Address>) -> Self {
        ...
    }
  
    // Returns whether the given movement is valid given the current game state
    fn is_valid(self, game_move: TicTacToeMove) -> Result<(), String> {
        ...
    }

    // Applies the move to the game object, transforming it
    fn apply_move(
      &mut self,
      game_move: &TicTacToeMove,
      player_index: usize,
      _author_address: &Address,
    ) -> () {
        ...
    }

    // Gets the winner for the game
    fn get_winner(
      &self,
      players: &Vec<Address>,
    ) -> Option<Address> {
        ...
    }
}
```

From now on, when calling most functions in the crate, we'll need to provide the game and move structs as type parameters so that the library can execute its functions.

### 4. Add the game and move entry definitions

``` rust
 #[entry_def]
fn game_def() -> ValidatingEntryType {
    holochain_turn_based_game::game_definition::<TicTacToe, TicTacToeMove>()
}

 #[entry_def]
fn move_def() -> ValidatingEntryType {
    holochain_turn_based_game::move_definition::<TicTacToe, TicTacToeMove>()
}
```

### 5. Add a receive callback to emit a signal when an opponent has moved

``` rust
#[receive]
fn receive(sender_address: Address, message: String) -> String {
    let result = holochain_turn_based_game::handle_receive_move(sender_address, message);

    JsonString::from(result).to_string()
}
```

## Play a game

### 1. Create a game

To create a game, call the `create_game` function:

``` rust
#[zome_fn("hc_public")]
fn create_game(rival: Address, timestamp: u32) -> ZomeApiResult<Address> {
    let game = GameEntry {
        players: vec![rival, hdk::AGENT_ADDRESS.clone()],
        created_at: timestamp,
    };

    holochain_turn_based_game::create_game(game)
}
```

The order of the players in the vector will determine the order in which they have to move.

### 2. Make a move

To create a move, call the `create_move` function:

``` rust
#[zome_fn("hc_public")]
fn place_piece(game_address: Address, x: usize, y: usize) -> ZomeApiResult<Address> {
    let game_move = TicTacToeMove::Place(Piece { x, y });

    holochain_turn_based_game::create_move(&game_address, game_move)
}
```

### 3. Get game state

To get the moves that have been done during the game, call `get_game_moves` :

``` rust
#[zome_fn("hc_public")]
fn get_moves(game_address: Address) -> ZomeApiResult<Vec<TicTacToeMove>> {
    holochain_turn_based_game::get_game_moves::<TicTacToe, TicTacToeMove>(&game_address)
}
```

And to get the winner of the game, call `get_game_winner` :

``` rust
#[zome_fn("hc_public")]
fn get_winner(game_address: Address) -> ZomeApiResult<Option<Address>> {
    holochain_turn_based_game::get_game_winner::<TicTacToe, TicTacToeMove>(&game_address)
}
```

To get the current game state, call `get_game_state` : 

``` rust
#[zome_fn("hc_public")]
fn get_game_state(game_address: Address) -> ZomeApiResult<TicTacToe> {
    holochain_turn_based_game::get_game_state::<TicTacToe, TicTacToeMove>(&game_address)
}
```
