//! This module represents the pieces that are being played in checkers
//! It includes the functionality of upgrading a piece type or dictating the type of movement a piece can do.

use crate::{
    err::{Error, Result},
    position::MovementDirection::{self, DownLeft, DownRight, UpLeft, UpRight},
    state::Player,
};

/// Represents a type of checkers piece
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceType {
    /// Has regular movement in the board
    Regular,

    /// Has special movement in the board
    King,
}

/// Represents playable piece in a checkers board
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Piece {
    /// Represents the owner of the piece
    pub owner: Player,

    /// Dictates the type of movement the piece can do
    pub piece_type: PieceType,
}

impl Piece {
    /// Calculates the valid movements of a piece, the move direction is absolute :
    /// * Black pieces are starting down and advancing up
    /// * White pieces are starting up and advancing down
    pub(crate) fn valid_move_directions(&self) -> Vec<MovementDirection> {
        if self.piece_type == PieceType::King {
            return vec![UpLeft, UpRight, DownLeft, DownRight];
        }

        match self.owner {
            Player::Black => vec![UpRight, UpLeft],
            Player::White => vec![DownRight, DownLeft],
        }
    }

    /// Upgrades a regular piece to a king, returns an error if the piece is already a king
    pub(crate) fn upgrade_to_king(&mut self) -> Result<()> {
        if self.piece_type == PieceType::King {
            return Err(Error::InvalidKingUpgrade);
        }

        self.piece_type = PieceType::King;

        Ok(())
    }
}

impl std::ops::Not for Player {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = match self {
            Piece {
                piece_type: PieceType::Regular,
                owner: Player::Black,
            } => "⚫️",
            Piece {
                piece_type: PieceType::King,
                owner: Player::Black,
            } => "🜲",
            Piece {
                piece_type: PieceType::Regular,
                owner: Player::White,
            } => "⚪️",
            Piece {
                piece_type: PieceType::King,
                owner: Player::White,
            } => "♛",
        };

        write!(f, "{piece}")?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_unordered::assert_eq_unordered;

    #[test]
    fn valid_white_move_directions() -> Result<()> {
        let piece = Piece {
            piece_type: PieceType::Regular,
            owner: Player::White,
        };

        assert_eq_unordered!(piece.valid_move_directions(), vec![DownRight, DownLeft]);

        Ok(())
    }

    #[test]
    fn valid_black_move_directions() -> Result<()> {
        let piece = Piece {
            piece_type: PieceType::Regular,
            owner: Player::Black,
        };

        assert_eq_unordered!(piece.valid_move_directions(), vec![UpRight, UpLeft]);

        Ok(())
    }

    #[test]
    fn valid_white_king_move_directions() -> Result<()> {
        let black_king = Piece {
            piece_type: PieceType::King,
            owner: Player::Black,
        };

        assert_eq_unordered!(
            black_king.valid_move_directions(),
            vec![DownLeft, DownRight, UpLeft, UpRight]
        );

        Ok(())
    }
    #[test]
    fn valid_black_king_move_directions() -> Result<()> {
        let white_king = Piece {
            piece_type: PieceType::King,
            owner: Player::White,
        };

        assert_eq_unordered!(
            white_king.valid_move_directions(),
            vec![DownLeft, DownRight, UpLeft, UpRight]
        );

        Ok(())
    }

    #[test]
    fn upgrade_to_king() -> Result<()> {
        let mut piece = Piece {
            piece_type: PieceType::Regular,
            owner: Player::Black,
        };
        piece.upgrade_to_king()?;
        assert_eq!(piece.piece_type, PieceType::King);

        let second_upgrade_result = piece.upgrade_to_king();
        assert!(matches!(
            second_upgrade_result,
            Err(Error::InvalidKingUpgrade)
        ));

        Ok(())
    }
}
