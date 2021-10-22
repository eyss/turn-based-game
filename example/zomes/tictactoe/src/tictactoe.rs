use hc_mixin_turn_based_game::{GameStatus, TurnBasedGame};
use hdk::prelude::holo_hash::AgentPubKeyB64;
use hdk::prelude::*;

pub const BOARD_SIZE: usize = 3;

#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct TicTacToe {
    pub player_1: (AgentPubKey, Vec<Piece>),
    pub player_2: (AgentPubKey, Vec<Piece>),
    pub player_resigned: Option<AgentPubKeyB64>,
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

impl TurnBasedGame for TicTacToe {
    type GameMove = TicTacToeMove;

    fn min_players() -> Option<usize> {
        Some(2)
    }

    fn max_players() -> Option<usize> {
        Some(2)
    }

    fn initial(players: Vec<AgentPubKeyB64>) -> Self {
        TicTacToe {
            player_1: (players[0].clone().into(), vec![]),
            player_2: (players[1].clone().into(), vec![]),
            player_resigned: None,
        }
    }

    fn apply_move(
        self,
        game_move: TicTacToeMove,
        author: AgentPubKeyB64,
    ) -> ExternResult<TicTacToe> {
        let mut game = self.clone();

        match game_move {
            TicTacToeMove::Place(piece) => {
                piece.is_in_bounds()?;
                piece.is_empty(&self)?;

                match author.eq(&self.player_1.0.clone().into()) {
                    true => game.player_1.1.push(piece.clone()),
                    false => game.player_2.1.push(piece.clone()),
                }
            }
            TicTacToeMove::Resign => game.player_resigned = Some(author),
        }

        Ok(game)
    }

    fn status(&self) -> GameStatus {
        if let Some(_) = self.player_resigned.clone() {
            return GameStatus::Finished;
        }

        if let Some(_) = self.winner() {
            return GameStatus::Finished;
        }
        return GameStatus::Ongoing;
    }
}

impl TicTacToe {
    pub fn to_dense(&self) -> [[u8; 8]; 8] {
        let mut board = [[0u8; 8]; 8];
        self.player_1.1.iter().for_each(|piece| {
            board[piece.x][piece.y] = 1;
        });
        self.player_2.1.iter().for_each(|piece| {
            board[piece.x][piece.y] = 2;
        });
        board
    }

    pub fn winner(&self) -> Option<u8> {
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
            return Some(0);
        } else if player_2_victory {
            return Some(2);
        } else {
            return None;
        }
    }
}
