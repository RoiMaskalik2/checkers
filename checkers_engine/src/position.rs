//! This module represents the positioning and basic rules of movement in checkers

use crate::{
    Error, Result,
    consts::{BOARD_SIZE, NEGATIVE_MOVEMENT, POSITIVE_MOVEMENT},
};

/// Represents a range of valid board positions (between 0-7 in both AXES of the board)
const VALID_POSITION_RANGE: std::ops::Range<u8> = 0..BOARD_SIZE;

/// Represents all valid movement directions in a checkers game
#[derive(Debug, PartialEq)]
pub enum MovementDirection {
    /// Move position up and right in the board
    UpRight,

    /// Move position up and left in the board
    UpLeft,

    /// Move position down and right in the board
    DownRight,

    /// Move position down and left in the board
    DownLeft,
}

/// A checkers board position will be represented by an absolute (x,y) point.
/// Meaning - It won't change with perspective on the board
/// Also, the board is a squared board, meaning there is an equal amount of cells in each axis
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    /// The absolute row in the board
    pub row: u8,

    /// The absolute column in the board
    pub column: u8,
}

impl Position {
    /// Calculates a new position given a movement direction
    /// Returns an error if the movement direction results in an invalid position
    pub fn step(&self, direction: MovementDirection) -> Result<Position> {
        let (row_change, column_change) = match direction {
            MovementDirection::DownLeft => (NEGATIVE_MOVEMENT, NEGATIVE_MOVEMENT),
            MovementDirection::DownRight => (NEGATIVE_MOVEMENT, POSITIVE_MOVEMENT),
            MovementDirection::UpLeft => (POSITIVE_MOVEMENT, NEGATIVE_MOVEMENT),
            MovementDirection::UpRight => (POSITIVE_MOVEMENT, POSITIVE_MOVEMENT),
        };
        let row = self
            .row
            .checked_add_signed(row_change)
            .ok_or(Error::InvalidPosition)?;

        let column = self
            .column
            .checked_add_signed(column_change)
            .ok_or(Error::InvalidPosition)?;

        let new_position = Position { row, column };

        Self::validate(new_position)?;

        Ok(new_position)
    }

    /// Validates a position
    /// Position is valid if it is not outside of a checkers board
    /// Meaning - both the axes of the position are between 0-7
    pub fn validate(position: Position) -> Result<()> {
        if !(VALID_POSITION_RANGE.contains(&position.row)
            && VALID_POSITION_RANGE.contains(&position.column))
        {
            return Err(Error::InvalidPosition);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_position() -> Result<()> {
        Position::validate(Position {
            row: 0,
            column: BOARD_SIZE - 1,
        })?;
        Ok(())
    }

    #[test]
    fn invalid_position() -> Result<()> {
        assert!(matches!(
            Position::validate(Position {
                row: 10,
                column: BOARD_SIZE - 1
            }),
            Err(Error::InvalidPosition)
        ));
        assert!(matches!(
            Position::validate(Position { row: 7, column: 10 }),
            Err(Error::InvalidPosition)
        ));

        Ok(())
    }

    #[test]
    fn step_up_right() -> Result<()> {
        let position = Position { row: 4, column: 4 };
        assert_eq!(
            position.step(MovementDirection::UpRight)?,
            Position { row: 5, column: 5 }
        );
        Ok(())
    }

    #[test]
    fn step_up_left() -> Result<()> {
        let position = Position { row: 4, column: 4 };
        assert_eq!(
            position.step(MovementDirection::UpLeft)?,
            Position { row: 5, column: 3 }
        );
        Ok(())
    }

    #[test]
    fn step_down_right() -> Result<()> {
        let position = Position { row: 4, column: 4 };
        assert_eq!(
            position.step(MovementDirection::DownRight)?,
            Position { row: 3, column: 5 }
        );
        Ok(())
    }

    #[test]
    fn step_down_left() -> Result<()> {
        let position = Position { row: 4, column: 4 };
        assert_eq!(
            position.step(MovementDirection::DownLeft)?,
            Position { row: 3, column: 3 }
        );
        Ok(())
    }

    #[test]
    fn successful_multi_step() -> Result<()> {
        let mut position = Position { row: 0, column: 0 };
        for _ in 0..BOARD_SIZE - 1 {
            position = position.step(MovementDirection::UpRight)?;
        }
        assert_eq!(
            position,
            Position {
                row: BOARD_SIZE - 1,
                column: BOARD_SIZE - 1
            }
        );

        for _ in 0..BOARD_SIZE - 1 {
            position = position.step(MovementDirection::DownLeft)?;
        }
        assert_eq!(position, Position { row: 0, column: 0 });
        Ok(())
    }

    #[test]
    fn fail_move_right_border_up_right() {
        let position = Position {
            row: BOARD_SIZE / 2,
            column: BOARD_SIZE - 1,
        };
        assert!(matches!(
            position.step(MovementDirection::UpRight),
            Err(Error::InvalidPosition)
        ));
    }

    #[test]
    fn fail_move_right_border_down_right() {
        let position = Position {
            row: BOARD_SIZE / 2,
            column: BOARD_SIZE - 1,
        };
        assert!(matches!(
            position.step(MovementDirection::DownRight),
            Err(Error::InvalidPosition)
        ));
    }

    #[test]
    fn fail_move_upper_border_up_left() {
        let position = Position {
            row: BOARD_SIZE - 1,
            column: BOARD_SIZE / 2,
        };
        assert!(matches!(
            position.step(MovementDirection::UpLeft),
            Err(Error::InvalidPosition)
        ));
    }

    #[test]
    fn fail_move_upper_border_up_right() {
        let position = Position {
            row: BOARD_SIZE - 1,
            column: BOARD_SIZE / 2,
        };
        assert!(matches!(
            position.step(MovementDirection::UpRight),
            Err(Error::InvalidPosition)
        ));
    }

    #[test]
    fn fail_move_left_border_up_left() {
        let position = Position {
            row: BOARD_SIZE / 2,
            column: 0,
        };
        assert!(matches!(
            position.step(MovementDirection::UpLeft),
            Err(Error::InvalidPosition)
        ));
    }

    #[test]
    fn fail_move_left_border_down_left() {
        let position = Position {
            row: BOARD_SIZE / 2,
            column: 0,
        };
        assert!(matches!(
            position.step(MovementDirection::DownLeft),
            Err(Error::InvalidPosition)
        ));
    }

    #[test]
    fn fail_move_lower_border_down_left() {
        let position = Position {
            row: 0,
            column: BOARD_SIZE / 2,
        };
        assert!(matches!(
            position.step(MovementDirection::DownLeft),
            Err(Error::InvalidPosition)
        ));
    }

    #[test]
    fn fail_move_lower_border_down_right() {
        let position = Position {
            row: 0,
            column: BOARD_SIZE / 2,
        };
        assert!(matches!(
            position.step(MovementDirection::DownRight),
            Err(Error::InvalidPosition)
        ));
    }
}
