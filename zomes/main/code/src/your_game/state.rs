use hdk::{
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
    AGENT_ADDRESS,
};

use super::MoveType;
use crate::game::Game;
use crate::game_move::Move;

/**
 *
 * As a game author you get to decide what the State object of your game looks like.
 * Most of the time you want it to include all of the previous moves as well.
 *
 * To customize the game state implement your own GameState struct. This must have a function called `initial()`
 * which returns the initial state.
 *
 */

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct GameState {
    pub moves: Vec<Move>,
    pub player_1_pieces: Vec<Piece>,
    pub player_2_pieces: Vec<Piece>,
    pub winner: Option<Address>,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub struct Piece {
    pub x: usize,
    pub y: usize,
}

const BOARD_SIZE: usize = 3;

impl Piece {
    pub fn is_in_bounds(&self) -> Result<(), String> {
        if self.x < BOARD_SIZE && self.y < BOARD_SIZE {
            Ok(())
        } else {
            Err("Piece is not in bounds".into())
        }
    }

    pub fn is_empty(&self, game_state: &GameState) -> Result<(), String> {
        if game_state.player_1_pieces.contains(self) || game_state.player_2_pieces.contains(self) {
            Err("Location is not empty".into())
        } else {
            Ok(())
        }
    }
}

impl GameState {
    pub fn initial() -> Self {
        Self {
            moves: Vec::new(),
            player_1_pieces: Vec::new(),
            player_2_pieces: Vec::new(),
            winner: None,
        }
    }

    pub fn render(&self) -> String {
        let turn: &str;
        if self.winner.is_some() {
            turn = "This game has finished"
        } else if let Some(last_move) = self.moves.last() {
            if last_move.author.to_string() != AGENT_ADDRESS.to_string() {
                turn = "It is your turn";
            } else {
                turn = "It is your opponent's turn";
            }
        } else {
            turn = "Player 2 goes first";
        }
        format!("{}\n\n{}", turn, Board::set(self).render())
    }

    pub fn evolve(&self, game: Game, next_move: &Move, is_last_move: bool) -> GameState {
        // given a current state, a game and a move, compute the next state
        // You can assume all moves are valid

        let mut moves = self.moves.clone();
        let mut player_1_pieces = self.player_1_pieces.clone();
        let mut player_2_pieces = self.player_2_pieces.clone();
        let mut winner = self.winner.clone();

        moves.push(next_move.clone());

        match next_move.move_type {
            MoveType::Place { x, y } => {
                // figure out which player made the move
                if game.player_1 == next_move.author {
                    player_1_pieces.push(Piece { x, y });
                    if is_last_move && Board::set(self).wins(x, y, 1) {
                        winner = Some(game.player_1);
                    }
                } else {
                    player_2_pieces.push(Piece { x, y });
                    if is_last_move && Board::set(self).wins(x, y, 2) {
                        winner = Some(game.player_2);
                    }
                }
            }
            MoveType::Resign => {
                if game.player_1 == next_move.author {
                    winner = Some(game.player_2)
                } else {
                    winner = Some(game.player_1)
                }
            }
        }

        GameState {
            moves,
            player_1_pieces,
            player_2_pieces,
            winner,
        }
    }
}

struct Board([[u8; BOARD_SIZE]; BOARD_SIZE]);

impl Board {
    fn set(game_state: &GameState) -> Self {
        let mut board = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        game_state.player_1_pieces
            .iter()
            .for_each(|Piece { x, y }| board[*y][*x] = 1);
        game_state.player_2_pieces
            .iter()
            .for_each(|Piece { x, y }| board[*y][*x] = 2);
        Self(board)
    }

    fn wins(&mut self, x: usize, y: usize, val: u8) -> bool {
        // set the potential win position
        self.0[y][x] = val;
        // look for horizontal win
        if self.0[y].iter().all(|&x| x == val) {
            return true;
        }
        // look for vertical win
        if self.0.iter().all(|&y| y[x] == val) {
            return true;
        }
        // look for main diagonal win
        if x == y && (0..BOARD_SIZE).all(|x| self.0[x][x] == val) {
            return true;
        }
        // look for anti diagonal win
        if x + y + 1 == BOARD_SIZE
            && (0..BOARD_SIZE).all(|x| {
                let y = BOARD_SIZE - x - 1;
                self.0[y][x] == val
            })
        {
            return true;
        }
        false
    }

    fn render(&self) -> String {
        let mut lines = "  x  0 1 2\ny\n".to_string();
        for (row, &y) in self.0.iter().enumerate() {
            lines.push_str(&format!("{}   ", row));
            for &x in y.iter() {
                lines.push_str(&format!(
                    "|{}",
                    match x {
                        2 => 'X',
                        1 => 'O',
                        _ => ' ',
                    }
                ));
            }
            lines.push_str("|\n");
        }
        lines
    }
}
