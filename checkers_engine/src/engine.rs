//! This module is the API for playing a checkers game.
//! It provides the following functionalities:
//! 1. Playing a turn with a chosen move
//! 2. Player turn and move validation
//! 3. Receiving the state of the game
//! 4. Receiving all of the cells of the board for external UI implementation

use crate::board::{Board, Cell, Move};
use crate::err::{Error, Result};
use crate::logic::{self, MoveType, ValidMove};
use crate::position::Position;
use crate::state::{Player, State, TurnState, WinState};

/// Provides all of the functionalities explained in the module documentation
pub struct CheckersEngine {
    current_state: State,
    board: Board,
    turns_since_last_eat: u8,
    cached_moves: Vec<ValidMove>,
}

impl CheckersEngine {
    /// Constructs a checkers board and initializes the state of the game.
    pub fn new() -> Self {
        let board = Board::new();
        let initial_state = State::NotFinished(TurnState::RegularTurn, Player::Black);

        Self {
            current_state: initial_state,
            board,
            turns_since_last_eat: 0,
            cached_moves: Vec::new(),
        }
    }

    /// Returns the state of the game.
    /// Useful for when some input was invalid for [`Self::play_turn`]
    /// and the implementor wants to get the state of the game after that.
    pub fn state(&self) -> State {
        self.current_state
    }

    /// The API that will allow to play a game of checkers.
    /// Performs multiple validations on the input and performs a move on the board.
    pub fn play_turn(&mut self, piece_move: Move, player: Player) -> Result<State> {
        if let State::GameOver(_) = self.current_state {
            return Err(Error::FinishedGame);
        }

        if logic::current_player(self.current_state)? != player {
            return Err(Error::InvalidPlayerTurn);
        }

        match self.board[piece_move.initial_position] {
            None => return Err(Error::MovingEmptyCell),
            Some(piece) if piece.owner != player => return Err(Error::MovingEnemyPiece),
            _ => {}
        }

        if self.cached_moves.is_empty() {
            self.cached_moves = logic::valid_moves(&self.board, self.current_state, player)?;
        }

        match self
            .cached_moves
            .iter()
            .find(|valid_move| valid_move.movement == piece_move)
            .copied()
        {
            Some(valid_move) => {
                logic::make_move(&mut self.board, valid_move)?;
                self.calculate_state(valid_move)
            }
            None => Err(Error::InvalidMove),
        }
    }

    /// Returns a vector of all of the cells that represent the board that is being played.
    pub fn board_cells(&self) -> Vec<(Position, Cell)> {
        self.board.cells()
    }

    // Updates the state of the game and turns counter,
    // and adds the next turn valid moves to the cache
    fn calculate_state(&mut self, piece_move: ValidMove) -> Result<State> {
        let current_player = logic::current_player(self.current_state)?;

        match piece_move.movement_type {
            MoveType::Eat(_) => self.turns_since_last_eat = 0,
            MoveType::Regular => self.turns_since_last_eat += 1,
        }

        let (new_state, new_moves) = logic::next_state_and_moves(
            &self.board,
            piece_move,
            current_player,
            self.turns_since_last_eat,
        )?;

        self.current_state = new_state;
        self.cached_moves = new_moves;

        Ok(self.current_state)
    }
}

impl Default for CheckersEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for CheckersEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_state = match self.current_state {
            State::GameOver(WinState::Draw) => "It's A Draw!",
            State::GameOver(WinState::Win(Player::Black)) => "Black Player Won!",
            State::GameOver(WinState::Win(Player::White)) => "White Player Won!",
            State::NotFinished(_, Player::Black) => &format!(
                "Black Player Turn: {} Turns With No Eat",
                self.turns_since_last_eat
            ),
            State::NotFinished(_, Player::White) => &format!(
                "White Player Turn: {} Turns With No Eat",
                self.turns_since_last_eat
            ),
        };

        let board = &self.board;

        writeln!(f, "{fmt_state}")?;
        writeln!(f, "{board}")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::BOARD_SIZE;

    #[test]
    fn game_starts_with_black_player() -> Result<()> {
        let engine = CheckersEngine::new();
        assert!(matches!(
            engine.state(),
            State::NotFinished(TurnState::RegularTurn, Player::Black)
        ));

        Ok(())
    }

    #[test]
    fn board_cells_returns_all_board_cells() -> Result<()> {
        let engine = CheckersEngine::new();
        assert_eq!(
            engine.board_cells().len(),
            (BOARD_SIZE * BOARD_SIZE) as usize
        );

        Ok(())
    }

    #[test]
    fn play_turn_player_validation() -> Result<()> {
        let mut engine = CheckersEngine::new();
        let result = engine.play_turn(
            Move {
                initial_position: Position { row: 5, column: 1 },
                target_position: Position { row: 4, column: 0 },
            },
            Player::White,
        );
        assert!(matches!(result, Err(Error::InvalidPlayerTurn)));

        Ok(())
    }

    #[test]
    fn play_turn_failed_moving_empty_cell() -> Result<()> {
        let mut engine = CheckersEngine::new();
        let result = engine.play_turn(
            Move {
                initial_position: Position { row: 3, column: 3 },
                target_position: Position { row: 4, column: 4 },
            },
            Player::Black,
        );
        assert!(matches!(result, Err(Error::MovingEmptyCell)));

        Ok(())
    }

    #[test]
    fn play_turn_failed_moving_enemy_piece() -> Result<()> {
        let mut engine = CheckersEngine::new();
        let result = engine.play_turn(
            Move {
                initial_position: Position { row: 5, column: 1 },
                target_position: Position { row: 4, column: 0 },
            },
            Player::Black,
        );
        assert!(matches!(result, Err(Error::MovingEnemyPiece)));

        Ok(())
    }

    #[test]
    fn play_turn_invalid_move_fails() -> Result<()> {
        let mut engine = CheckersEngine::new();
        let result = engine.play_turn(
            Move {
                initial_position: Position { row: 2, column: 0 },
                target_position: Position { row: 2, column: 2 },
            },
            Player::Black,
        );
        assert!(matches!(result, Err(Error::InvalidMove)));

        Ok(())
    }

    #[test]
    fn play_turn_valid_move_switches_to_enemy_turn() -> Result<()> {
        let mut engine = CheckersEngine::new();
        let state = engine.play_turn(
            Move {
                initial_position: Position { row: 2, column: 0 },
                target_position: Position { row: 3, column: 1 },
            },
            Player::Black,
        )?;
        assert!(matches!(
            state,
            State::NotFinished(TurnState::RegularTurn, Player::White)
        ));

        Ok(())
    }

    #[test]
    fn state_matches_last_play_turn_result() -> Result<()> {
        let mut engine = CheckersEngine::new();
        engine.play_turn(
            Move {
                initial_position: Position { row: 2, column: 0 },
                target_position: Position { row: 3, column: 1 },
            },
            Player::Black,
        )?;

        assert!(matches!(
            engine.state(),
            State::NotFinished(TurnState::RegularTurn, Player::White)
        ));

        Ok(())
    }
}
