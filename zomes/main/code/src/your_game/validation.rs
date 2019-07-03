use super::state::Piece;
use super::GameState;
use crate::{game::Game, game_move::Move, your_game::MoveType};
use hdk::holochain_persistence_api::cas::content::Address;

/**
 *
 * To implement your own custom rule validation all you need to do is re-implement the function `is_valid` on `Move`
 *
 * This function  takes the current game and the game state (which includes all the existing moves)
 * and determines if a new candidate move is valid. Typically this will involve first matching on the move type
 * and then determining if the move is valid.
 *
 * It function must return Ok(()) if a move is valid and Err("Some error string") for an invalid move.
 * It is useful to provide descriptive error strings as these can be visible to the end user.
 *
 */

impl Move {
    pub fn is_valid(&self, game: Game, game_state: GameState) -> Result<(), String> {
        is_players_turn(self.author.clone(), &game, &game_state)?;
        match self.move_type {
            MoveType::Place { x, y } => {
                let position = Piece { x, y };
                position.is_in_bounds()?;
                position.is_empty(&game_state)?;
                position.can_capture(&game, &game_state)?;
                Ok(())
            }
            MoveType::Pass => Ok(()),
            MoveType::Resign => Ok(()),
        }
    }
}

fn is_players_turn(player: Address, game: &Game, game_state: &GameState) -> Result<(), String> {
    if game_state.game_over {
        return Err("This game has ended".into());
    }

    let moves = &game_state.moves;
    match moves.last() {
        Some(last_move) => {
            if last_move.author == player {
                Err("It is not this player's turn".into())
            } else {
                Ok(())
            }
        }
        None => {
            // also need to handle the case where no moves have been made yet
            if game.player_2 == player {
                Ok(()) // player 2 can go first by convention
            } else {
                Err("Player 2 must make the first move".into())
            }
        }
    }
}
