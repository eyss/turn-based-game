use hdk::prelude::*;
use holochain_turn_based_game::game::Game;

pub const BOARD_SIZE: usize = 3;

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct TicTacToe {
  pub player_1_address: Address,
  pub player_1: Vec<Piece>,
  pub player_2_address: Address,
  pub player_2: Vec<Piece>,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub enum TicTacToeMove {
  Place(Piece),
  Resign,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Piece {
  pub x: usize,
  pub y: usize,
}

impl Piece {
  fn is_in_bounds(&self) -> Result<(), String> {
    if self.x < BOARD_SIZE && self.y < BOARD_SIZE
    // no need to check > 0 as usize is always positive
    {
      Ok(())
    } else {
      Err("Position is not in bounds".to_string())
    }
  }

  fn is_empty(&self, game_state: &TicTacToe) -> Result<(), String> {
    match board_sparse_to_dense(game_state)[self.x][self.y] == 0 {
      true => Ok(()),
      false => Err("A piece already exists at that position.".to_string()),
    }
  }
}

impl Game<TicTacToeMove> for TicTacToe {
  fn min_players() -> Option<usize> {
    Some(2)
  }

  fn max_players() -> Option<usize> {
    Some(2)
  }

  fn initial(players: &Vec<Address>) -> Self {
    TicTacToe {
      player_1_address: players[0],
      player_2_address: players[1],
      player_1: vec![],
      player_2: vec![],
    }
  }

  fn is_valid(self, game_move: TicTacToeMove) -> Result<(), String> {
    match game_move {
      TicTacToeMove::Place(piece) => {
        pos.is_in_bounds()?;
        pos.is_empty(&game_state)?;

        Ok(())
      }
      TicTacToeMove::Resign => Ok(()),
    }
  }

  fn apply_move(&mut self, author_address: &Address, game_move: &TicTacToeMove) -> () {
    if let TicTacToeMove::Place(piece) = game_move {
      if author_address == self.player_1_address {
        self.player_1.push(piece);
      } else {
        self.player_2.push(piece);
      }
    }
  }

  fn get_winner(&self, moves: &Vec<TicTacToeMove>) -> Option<Address> {
    if let Some(TicTacToeMove::Resign) = moves.last() {}
    self
  }
}

impl TicTacToe {
  pub fn to_dense(&self) -> [[u8; 8]; 8] {
    let mut board = [[0u8; 8]; 8];
    self.player_1.pieces.iter().for_each(|piece| {
      board[piece.x][piece.y] = 1;
    });
    self.player_2.pieces.iter().for_each(|piece| {
      board[piece.x][piece.y] = 2;
    });
    board
  }

  pub fn from_dense(board: [[u8; 8]; 8]) -> Self {
    let mut player_1_pieces = Vec::new();
    let mut player_2_pieces = Vec::new();
    board.iter().enumerate().for_each(|(x, row)| {
      row.iter().enumerate().for_each(|(y, square)| {
        if *square == 1 {
          player_1_pieces.push(Piece { x, y });
        } else if *square == 2 {
          player_2_pieces.push(Piece { x, y });
        }
      })
    });

    TicTacToe {
      player_1: player_1_pieces,
      player_2: player_2_pieces,
    }
  }
}
