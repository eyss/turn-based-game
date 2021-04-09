use hdk::prelude::*;
use holochain_turn_based_game::prelude::TurnBasedGame;

pub const BOARD_SIZE: usize = 3;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TicTacToe {
    pub player_1: Vec<Piece>,
    pub player_2: Vec<Piece>,
    pub player_resigned: Option<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub enum TicTacToeMove {
    Place(Piece),
    Resign,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Piece {
    pub x: usize,
    pub y: usize,
}

impl Piece {
    fn is_in_bounds(&self) -> ExternResult<()> {
        if self.x < BOARD_SIZE && self.y < BOARD_SIZE
        // no need to check > 0 as usize is always positive
        {
            Ok(())
        } else {
            Err(WasmError::Guest("Position is not in bounds".into()))
        }
    }

    fn is_empty(&self, game_state: &TicTacToe) -> ExternResult<()> {
        match game_state.to_dense()[self.x][self.y] == 0 {
            true => Ok(()),
            false => Err(WasmError::Guest(
                "A piece already exists at that position".into(),
            )),
        }
    }
}

impl TurnBasedGame<TicTacToeMove> for TicTacToe {
    fn min_players() -> Option<usize> {
        Some(2)
    }

    fn max_players() -> Option<usize> {
        Some(2)
    }

    fn initial(_players: &Vec<AgentPubKey>) -> Self {
        TicTacToe {
            player_1: vec![],
            player_2: vec![],
            player_resigned: None,
        }
    }

    fn apply_move(
        &mut self,
        game_move: &TicTacToeMove,
        _players: &Vec<AgentPubKey>,
        author_index: usize,
    ) -> ExternResult<()> {
        match game_move {
            TicTacToeMove::Place(piece) => {
                piece.is_in_bounds()?;
                piece.is_empty(&self)?;
                match author_index {
                    0 => self.player_1.push(piece.clone()),
                    1 => self.player_2.push(piece.clone()),
                    _ => {}
                }
            }
            TicTacToeMove::Resign => self.player_resigned = Some(author_index as u8),
        }

        Ok(())
    }

    fn get_winner(&self, players: &Vec<AgentPubKey>) -> Option<AgentPubKey> {
        if let Some(resigned_player) = self.player_resigned {
            let winner_index = if resigned_player == 0 { 1 } else { 0 };
            return Some(players[winner_index].clone());
        }

        let board = self.to_dense();

        // check if this resulted in a player victory
        let mut diag_down = 0;
        let mut diag_up = 0;
        let mut across = [0; 3];
        let mut down = [0; 3];
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let delta = match board[x][y] {
                    1 => 1,
                    2 => -1,
                    _ => 0,
                };
                down[x] += delta;
                across[y] += delta;
                // diag down e.g. \
                if x == y {
                    diag_down += delta;
                }
                //diag up  e.g. /
                else if x == (BOARD_SIZE - 1 - y) {
                    diag_up += delta;
                }
            }
        }
        let player_1_victory = across.iter().any(|e| *e == (BOARD_SIZE as i32))
            || down.iter().any(|e| *e == (BOARD_SIZE as i32))
            || diag_down == (BOARD_SIZE as i32)
            || diag_up == (BOARD_SIZE as i32);

        let player_2_victory = across.iter().any(|e| *e == (-1 * BOARD_SIZE as i32))
            || down.iter().any(|e| *e == (-1 * BOARD_SIZE as i32))
            || diag_down == (-1 * BOARD_SIZE as i32)
            || diag_up == (-1 * BOARD_SIZE as i32);
        if player_1_victory {
            return Some(players[0].clone());
        } else if player_2_victory {
            return Some(players[1].clone());
        }
        return None;
    }
}

impl TicTacToe {
    pub fn to_dense(&self) -> [[u8; 8]; 8] {
        let mut board = [[0u8; 8]; 8];
        self.player_1.iter().for_each(|piece| {
            board[piece.x][piece.y] = 1;
        });
        self.player_2.iter().for_each(|piece| {
            board[piece.x][piece.y] = 2;
        });
        board
    }

    pub fn _from_dense(board: [[u8; 8]; 8]) -> Self {
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
            player_resigned: None,
        }
    }
}
