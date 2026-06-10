//! This module represents the pieces that are being played in checkers
//! It includes the functionality of upgrading a piece type or dictating the type of movement a piece can do.

use crate::{
    Error,
    MovementDirection::{self, DownLeft, DownRight, UpLeft, UpRight},
    Result,
};

/// Represents a type of checkers piece
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceType {
    /// Has regular movement in the board
    Regular,

    /// Has special movement in the board
    King,
}

/// Represents the type of players that play a checkers game
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    /// Has black checkers pieces
    Black,

    /// Has white checkers pieces
    White,
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
    pub fn valid_move_directions(&self) -> Vec<MovementDirection> {
        if let PieceType::King = self.piece_type {
            return vec![UpLeft, UpRight, DownLeft, DownRight];
        }

        match self.owner {
            Player::Black => vec![DownRight, DownLeft],
            Player::White => vec![UpRight, UpLeft],
        }
    }

    /// Upgrades a regular piece to a king, returns an error if the piece is already a king
    pub fn upgrade_to_king(&mut self) -> Result<()> {
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

        assert_eq_unordered!(piece.valid_move_directions(), vec![UpRight, UpLeft]);

        Ok(())
    }

    #[test]
    fn valid_black_move_directions() -> Result<()> {
        let piece = Piece {
            piece_type: PieceType::Regular,
            owner: Player::Black,
        };

        assert_eq_unordered!(piece.valid_move_directions(), vec![DownLeft, DownRight]);

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
    fn not_player() -> Result<()> {
        let player = !Player::White;
        assert_eq!(player, Player::Black);

        let player = !Player::Black;
        assert_eq!(player, Player::White);

        Ok(())
    }

    #[test]
    fn upgrade_to_king() -> Result<()> {
        let mut piece = Piece {
            piece_type: PieceType::Regular,
            owner: Player::Black,
        };
        piece.upgrade_to_king()?;

        let second_upgrade_result = piece.upgrade_to_king();
        assert!(matches!(
            second_upgrade_result,
            Err(Error::InvalidKingUpgrade)
        ));

        Ok(())
    }
}
