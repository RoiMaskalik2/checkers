//! This module represents a checker board
//! A checkers board position will be represented by an absolute (x,y) point.
//! Meaning - It won't change with perspective on the board
//! Also, the board is a squared board, meaning there is an equal amount of cells in each axis

use crate::{
    consts::BOARD_SIZE,
    piece::{Piece, PieceType},
    position::Position,
    state::Player,
};

use std::array;

/// Represents a position that will grant access to a cell on the board.
pub type Cell = Option<Piece>;

/// Each piece type starts with 3 full rows
const STARTING_PIECE_ROWS: u8 = 3;

/// The bottom three rows in the board are the initial position of Black pieces
const STARTING_BLACK_ROWS: u8 = 0;

/// The top three rows in the board are the initial position of White pieces
const STARTING_WHITE_ROWS: u8 = BOARD_SIZE - STARTING_PIECE_ROWS;

/// Used In a calculation with a board row,column:
/// If their sum has a modulu of 0, than it is a black square - meaning a square with a possible piece
const BLACK_CHECKERS_SQUARES: u8 = 0;

/// Represents the sum of row and column indexes in board
/// which will determine the color of the checkers board
const SQUARE_COLOR_MODULU: u8 = 2;

/// Represents a checkers board container
/// NOTE: a board will not contain checkers logic and will be used as a container of all of the data.
#[derive(Debug)]
pub(crate) struct Board {
    /// None Represents that a board cell is empty and does not contain a piece
    board: [[Cell; BOARD_SIZE as usize]; BOARD_SIZE as usize],
}

/// Represents a single move on a checkers board.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Move {
    /// Initial position
    pub initial_position: Position,

    /// Target position
    pub target_position: Position,
}

impl Board {
    /// Initializes a board to starting checkers state
    pub(crate) fn new() -> Self {
        Self::initialize_checkers_board()
    }

    /// Initializes a board with no pieces
    #[cfg(test)]
    pub(crate) fn empty() -> Self {
        Self {
            board: [[None; BOARD_SIZE as usize]; BOARD_SIZE as usize],
        }
    }

    /// Takes a [`Move`], "cuts" the cell on the initial position of the move
    /// And overrides the cell in the target position with it.
    /// **NOTE: ** This function does not have logic
    /// Meaning it will override the target even if there is some piece in there
    /// And it will also move the initial position even if there is no piece in there
    pub(crate) fn move_cell(&mut self, board_move: Move) {
        let moved_cell = self.pop_cell(board_move.initial_position);
        self[board_move.target_position] = moved_cell;
    }

    /// Returns a vector of all of the cells that represents this board
    pub(crate) fn cells(&self) -> Vec<(Position, Cell)> {
        (0..BOARD_SIZE)
            .flat_map(|row| {
                (0..BOARD_SIZE)
                    .map(move |column| Position { row, column })
                    .map(|position| (position, self[position]))
            })
            .collect()
    }

    /// Overrides the cell of the given position with an empty cell (None)
    pub(crate) fn clear_cell(&mut self, position: Position) {
        self[position] = None;
    }

    // Implemented to keep the "new" constructor clean
    fn initialize_checkers_board() -> Self {
        let board = array::from_fn(|row| {
            array::from_fn(|column| Self::initialize_checkers_piece(row as u8, column as u8))
        });

        Self { board }
    }

    // Helper of initialize_checkers_board.
    // used for initializing a piece on a specific location on the board
    fn initialize_checkers_piece(row: u8, column: u8) -> Cell {
        if (row + column) % SQUARE_COLOR_MODULU != BLACK_CHECKERS_SQUARES {
            return None;
        }

        let owner = match row {
            STARTING_BLACK_ROWS..STARTING_PIECE_ROWS => Player::Black,
            STARTING_WHITE_ROWS..BOARD_SIZE => Player::White,
            _ => return None,
        };

        Some(Piece {
            owner,
            piece_type: PieceType::Regular,
        })
    }

    /// Overrides the cell of the given position with an empty cell (None)
    /// And returns the cell that was previously on there
    fn pop_cell(&mut self, position: Position) -> Cell {
        self[position].take()
    }
}

impl std::ops::Index<Position> for Board {
    type Output = Cell;
    /// Returns a board cell given a position on the board
    fn index(&self, position: Position) -> &Self::Output {
        &self.board[position.row as usize][position.column as usize]
    }
}

impl std::ops::IndexMut<Position> for Board {
    /// Returns a mutable board cell given a position on the board
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        &mut self.board[position.row as usize][position.column as usize]
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const BLACK_SQUARE: &str = "\x1b[48;2;62;96;111m";
        const WHITE_SQUARE: &str = "\x1b[48;2;193;198;200m";
        const RESET_FORMAT: &str = "\x1b[0m";

        write!(f, "    ")?;
        for column_index in 0..BOARD_SIZE {
            write!(f, " {column_index} ")?;
        }
        writeln!(f)?;

        for row in 0..BOARD_SIZE {
            write!(f, "{row}   ")?;
            for column in 0..BOARD_SIZE {
                if (row + column) % SQUARE_COLOR_MODULU == BLACK_CHECKERS_SQUARES {
                    let piece = match self[Position { row, column }] {
                        None => "  ",
                        Some(piece) => &format!("{piece}"),
                    };

                    write!(f, "{BLACK_SQUARE}{piece} {RESET_FORMAT}")?;
                } else {
                    write!(f, "{WHITE_SQUARE}   {RESET_FORMAT}")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cells_returns_all_board_positions() {
        let board = Board::new();
        assert_eq!(board.cells().len(), (BOARD_SIZE * BOARD_SIZE) as usize);
    }

    #[test]
    fn new_board_empty_cell_at_non_black_cell() {
        let board = Board::new();
        assert_eq!(board[Position { row: 0, column: 1 }], None);
    }

    #[test]
    fn new_board_middle_rows_are_empty() {
        let board = Board::new();
        for column in 0..BOARD_SIZE {
            assert_eq!(
                board[Position {
                    row: STARTING_PIECE_ROWS,
                    column
                }],
                None,
                "{STARTING_PIECE_ROWS}, {column}"
            );
            assert_eq!(
                board[Position {
                    row: STARTING_WHITE_ROWS - 1,
                    column
                }],
                None,
                "{STARTING_PIECE_ROWS}, {column}"
            );
        }
    }

    #[test]
    fn new_board_black_pieces_only_in_black_starting_rows() {
        let board = Board::new();
        for (position, cell) in board.cells() {
            if let Some(piece) = cell {
                if piece.owner == Player::Black {
                    assert!(position.row < STARTING_PIECE_ROWS, "{position:?}");
                }
            }
        }
    }

    #[test]
    fn new_board_white_pieces_only_in_white_starting_rows() {
        let board = Board::new();
        for (position, cell) in board.cells() {
            if let Some(piece) = cell {
                if piece.owner == Player::White {
                    assert!(position.row >= STARTING_WHITE_ROWS, "{position:?}");
                }
            }
        }
    }

    #[test]
    fn new_board_all_starting_pieces_are_regular() {
        let board = Board::new();
        for (position, cell) in board.cells() {
            if let Some(piece) = cell {
                assert_eq!(piece.piece_type, PieceType::Regular, "{position:?}");
            }
        }
    }

    #[test]
    fn move_cell_overrides_piece_at_target() {
        let mut board = Board::new();
        let initial_position = Position { row: 0, column: 0 };
        let target_position = Position { row: 1, column: 2 };
        let expected = board[initial_position];

        // make sure the target was something other than the source
        assert_ne!(board[initial_position], board[target_position]);

        board.move_cell(Move {
            initial_position,
            target_position,
        });

        assert_eq!(board[target_position], expected);
    }

    #[test]
    fn move_cell_clears_initial_position() {
        let mut board = Board::new();
        let initial_position = Position { row: 0, column: 0 };
        let target_position = Position { row: 1, column: 1 };

        // make sure there was actually a piece before the move
        assert_ne!(board[initial_position], None);

        board.move_cell(Move {
            initial_position,
            target_position,
        });

        assert_eq!(board[initial_position], None);
    }
}
