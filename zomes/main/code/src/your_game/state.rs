use hdk::{
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
    AGENT_ADDRESS,
};
use std::cmp::{max, min};

use super::MoveType;
use crate::{game::Game, game_move::Move};

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
    pub game_over: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
pub struct Piece {
    pub x: usize,
    pub y: usize,
}

const BOARD_SIZE: usize = 8;

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

    pub fn can_capture(&self, game: &Game, game_state: &GameState) -> Result<(), String> {
        let player_num = if AGENT_ADDRESS.to_string() == game.player_1.to_string() {
            1
        } else {
            2
        };
        if !Board::set(game_state).can_capture(self.x, self.y, player_num) {
            Err("No opponent pieces can be flipped from this location".into())
        } else {
            Ok(())
        }
    }
}

impl GameState {
    pub fn initial() -> Self {
        Self {
            moves: Vec::new(),
            player_1_pieces: vec![Piece { x: 4, y: 3 }, Piece { x: 3, y: 4 }],
            player_2_pieces: vec![Piece { x: 3, y: 3 }, Piece { x: 4, y: 4 }],
            winner: None,
            game_over: false,
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
        let mut game_over = self.game_over.clone();

        moves.push(next_move.clone());

        match next_move.move_type {
            MoveType::Place { x, y } => {
                // figure out which player made the move
                if game.player_1 == next_move.author {
                    player_1_pieces.push(Piece { x, y });
                    if is_last_move {
                        let captured = Board::set(self).do_capture(x, y, 1);
                        player_2_pieces = player_2_pieces
                            .into_iter()
                            .filter(|piece| !captured.contains(piece))
                            .collect();
                        captured
                            .into_iter()
                            .for_each(|piece| player_1_pieces.push(piece));
                    }
                } else {
                    player_2_pieces.push(Piece { x, y });
                    if is_last_move {
                        let captured = Board::set(self).do_capture(x, y, 2);
                        player_1_pieces = player_1_pieces
                            .into_iter()
                            .filter(|piece| !captured.contains(piece))
                            .collect();
                        captured
                            .into_iter()
                            .for_each(|piece| player_2_pieces.push(piece));
                    }
                }
            }
            MoveType::Pass => {
                if let Some(last_move) = moves.last() {
                    if last_move.move_type == MoveType::Pass {
                        if player_1_pieces.len() > player_2_pieces.len() {
                            winner = Some(game.player_1);
                        } else if player_2_pieces.len() > player_1_pieces.len() {
                            winner = Some(game.player_2);
                        }
                        game_over = true;
                    }
                }
            }
            MoveType::Resign => {
                if game.player_1 == next_move.author {
                    winner = Some(game.player_2);
                    game_over = true;
                } else {
                    winner = Some(game.player_1);
                    game_over = true;
                }
            }
        }

        GameState {
            moves,
            player_1_pieces,
            player_2_pieces,
            winner,
            game_over,
        }
    }
}

struct Board([[u8; BOARD_SIZE]; BOARD_SIZE]);

impl Board {
    fn set(game_state: &GameState) -> Self {
        let mut board = [[0u8; BOARD_SIZE]; BOARD_SIZE];
        game_state
            .player_1_pieces
            .iter()
            .for_each(|Piece { x, y }| board[*y][*x] = 1);
        game_state
            .player_2_pieces
            .iter()
            .for_each(|Piece { x, y }| board[*y][*x] = 2);
        Self(board)
    }

    fn can_capture(&self, x: usize, y: usize, player: u8) -> bool {
        self.may_capture(x,y,player,false).1
    }

    fn do_capture(&mut self, x: usize, y: usize, player: u8) -> Vec<Piece> {
        self.may_capture(x,y,player,true).0
    }

    fn may_capture(&self, x: usize, y: usize, player: u8, do_capture: bool) -> (Vec<Piece>, bool) {
        let board = &self.0;
        let opponent = 3 - player;
        let mut flipped: Vec<Piece> = Vec::new();
        // horizontal forward
        if x + 2 < BOARD_SIZE && board[y][x + 1] == opponent {
            let mut iter = board[y][x + 2..BOARD_SIZE]
                .iter()
                .skip_while(|&&o| o == opponent)
                .enumerate();
            if let Some((n, &p)) = iter.next() {
                if p == player {
                    if do_capture {
                        (x + 1..x + n + 2).for_each(|x| flipped.push(Piece { x, y }));
                    } else {
                        return (flipped, true);
                    }
                }
            }
        }
        // horizontal backward
        if x >= 2 && board[y][x - 1] == opponent {
            let mut iter = board[y][0..x - 1]
                .iter()
                .rev()
                .skip_while(|&&x| x == opponent)
                .enumerate();
            if let Some((n, &p)) = iter.next() {
                if p == player {
                    if do_capture {
                        (x - n - 2..x - 1).for_each(|x| flipped.push(Piece { x, y }));
                    } else {
                        return (flipped, true);
                    }
                }
            }
        }
        // vertical downward
        if y + 2 < BOARD_SIZE && board[y + 1][x] == opponent {
            let mut iter = board[y + 2..BOARD_SIZE]
                .iter()
                .skip_while(|&&y| y[x] == opponent)
                .enumerate();
            if let Some((n, &p)) = iter.next() {
                if p[x] == player {
                    if do_capture {
                        (y + 1..y + n + 2).for_each(|y| flipped.push(Piece { x, y }));
                    } else {
                        return (flipped, true);
                    }
                }
            }
        }
        // vertical upward
        if y >= 2 && board[y - 1][x] == opponent {
            let mut iter = board[0..y - 1]
                .iter()
                .rev()
                .skip_while(|&&y| y[x] == opponent)
                .enumerate();
            if let Some((n, &p)) = iter.next() {
                if p[x] == player {
                    if do_capture {
                        (y - n - 2..y - 1).for_each(|y| flipped.push(Piece { x, y }));
                    } else {
                        return (flipped, true);
                    }
                }
            }
        }
        // downward diagonal forward
        if max(x, y) + 2 < BOARD_SIZE && board[y + 1][x + 1] == opponent {
            let mut iter = (2..BOARD_SIZE - max(x, y))
                .skip_while(|&d| board[y + d][x + d] == opponent)
                .enumerate();
            if let Some((n, d)) = iter.next() {
                if board[y + d][x + d] == player {
                    if do_capture {
                        (1..n + 1).for_each(|d| flipped.push(Piece { x: x + d, y: y + d }));
                    } else {
                        return (flipped, true);
                    }
                }
            }
        }
        // downward diagonal backward
        if min(x, y) >= 2 && board[y - 1][x - 1] == opponent {
            let mut iter = (2..min(x, y))
                .skip_while(|&d| board[y - d][x - d] == opponent)
                .enumerate();
            if let Some((n, d)) = iter.next() {
                if board[y - d][x - d] == player {
                    if do_capture {
                        (1..n + 1).for_each(|d| flipped.push(Piece { x: x - d, y: y - d }));
                    } else {
                        return (flipped, true);
                    }
                }
            }
        }
        // upward diagonal forward
        if x + 2 < BOARD_SIZE && y >= 2 && board[y - 1][x + 1] == opponent {
            let mut iter = (2..min(BOARD_SIZE - x, y))
                .skip_while(|&d| board[y - d][x + d] == opponent)
                .enumerate();
            if let Some((n, d)) = iter.next() {
                if board[y - d][x + d] == player {
                    if do_capture {
                        (1..n + 1).for_each(|d| flipped.push(Piece { x: x + d, y: y - d }));
                    } else {
                        return (flipped, true);
                    }
                }
            }
        }
        // upward diagonal backward
        if x >= 2 && y + 2 < BOARD_SIZE && board[y + 1][x - 1] == opponent {
            let mut iter = (2..min(x, BOARD_SIZE - y))
                .skip_while(|&d| board[y + d][x - d] == opponent)
                .enumerate();
            if let Some((n, d)) = iter.next() {
                if board[y + d][x - d] == player {
                    if do_capture {
                        (1..n + 1).for_each(|d| flipped.push(Piece { x: x - d, y: y + d }));
                    } else {
                        return (flipped, true);
                    }
                }
            }
        }
        (flipped, false)
    }

    fn render(&self) -> String {
        let mut lines = "  x  0 1 2 3 4 5 6 7\ny\n".to_string();
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
