use hdk::holochain_core_types::{
    error::HolochainError,
    json::JsonString,
};

use crate::game_move::Move;
use crate::checkers::{
    moves::Piece,
    MoveType,
};


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
    pub complete: bool,
    pub moves: Vec<Move>,
    pub player_1: PlayerState,
    pub player_2: PlayerState,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct PlayerState {
    pub pieces: Vec<Piece>,
    pub resigned: bool,
}


impl GameState {
    pub fn initial() -> Self {
        let p1 = PlayerState {
            pieces: vec![
                Piece{x: 0, y: 0}, Piece{x: 2, y: 0}, Piece{x: 4, y: 0}, Piece{x: 6, y: 0},
                Piece{x: 1, y: 1}, Piece{x: 3, y: 1}, Piece{x: 5, y: 1}, Piece{x: 7, y: 1},
                Piece{x: 0, y: 2}, Piece{x: 2, y: 2}, Piece{x: 4, y: 2}, Piece{x: 6, y: 2},
            ],
            resigned: false,
        };
        let p2 = PlayerState {
            pieces: vec![
                Piece{x: 1, y: 5}, Piece{x: 3, y: 5}, Piece{x: 5, y: 5}, Piece{x: 7, y: 5},
                Piece{x: 0, y: 6}, Piece{x: 2, y: 6}, Piece{x: 4, y: 6}, Piece{x: 6, y: 6},
                Piece{x: 1, y: 7}, Piece{x: 3, y: 7}, Piece{x: 5, y: 7}, Piece{x: 7, y: 7},
            ],
            resigned: false,
        };
        GameState {
            moves: Vec::new(),
            complete: false,
            player_1: p1,
            player_2: p2,
        }
    }
}

/// takes a current game state and a move and progresses the state
/// assumes that moves are totally valid by this stage
pub fn state_reducer(current_state: GameState, next_move: &Move) -> GameState {
    match &next_move.move_type {
        MoveType::MovePiece{to, from} => {
            let mut board = board_sparse_to_dense(&current_state);
            let mut moves = current_state.moves;
            moves.push(next_move.to_owned());
            // make the move by deleting the piece at the from position and adding one at the to position
            board[from.x][from.y] = 0;
            board[to.x][to.y] = 1;

            // check if any opponent pieces were taken in this move

            let (player_1_pieces, player_2_pieces) = board_dense_to_sparse(board);

            GameState{
                player_1: PlayerState {
                    pieces: player_1_pieces,
                    resigned: false,
                },
                player_2: PlayerState {
                    pieces: player_2_pieces,
                    resigned: false,
                },
                moves,
                ..current_state
            }
        }
    }
}

/*========================================
=            Helper functions            =
========================================*/

fn board_sparse_to_dense(state: &GameState)-> [[u8; 8]; 8] {
    let mut board = [[0u8; 8]; 8];
    state.player_1.pieces.iter().for_each(|piece| {
        board[piece.x][piece.y] = 1;
    });
    state.player_2.pieces.iter().for_each(|piece| {
        board[piece.x][piece.y] = 2;
    });
    board
}

fn board_dense_to_sparse(board: [[u8; 8]; 8]) -> (Vec<Piece>, Vec<Piece>) {
    let mut player_1_pieces = Vec::new();
    let mut player_2_pieces = Vec::new();
    board.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, square)| {
            if *square == 1 {
                player_1_pieces.push(Piece{x , y});
            } else if *square == 2 {
                player_2_pieces.push(Piece{x , y});               
            }
        })
    });
    (player_1_pieces, player_2_pieces)
}

/*=====  End of Helper functions  ======*/
