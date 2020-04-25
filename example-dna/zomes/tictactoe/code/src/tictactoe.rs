use hdk::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct TicTacToe {
  pub player_1: Vec<Piece>,
  pub player_2: Vec<Piece>,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct TicTacToeMove {}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Piece {
  pub x: usize;
  pub y: usize;
}
