//! Errors that can occur in this crate, grouped by the module they came from.

/// Represents an error that can occur while using the checkers engine crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // ---- position module --------------------------------------------
    /// Position is not in the valid board range (between 0-7 in both AXES of the board)
    #[error("{self:?}")]
    InvalidPosition,

    // ---- piece module --------------------------------------------
    /// An attempt to convert a piece to a king where the piece was already a king
    #[error("{self:?}")]
    InvalidKingUpgrade,

    // ---- engine module --------------------------------------------
    /// An attempt to play a checkers game when the game has finished
    #[error("Error: The game has finished and no additional moves can be performed")]
    FinishedGame,
    /// An attempt to play a checkers turn with the wrong player
    #[error("Error: This player should not play this turn")]
    InvalidPlayerTurn,
    /// An attempt to perform a move that is not valid in checkers
    #[error("Error: This move is not a valid move")]
    InvalidMove,
    /// An attempt to move from a board cell that does not contain a piece
    #[error("Error: You have tried to move a piece that does not exist")]
    MovingEmptyCell,
    /// An attempt to move a piece that is not owned by the current player
    #[error("Error: You have tried to move an enemy piece")]
    MovingEnemyPiece,

    // ---- logic module --------------------------------------------
    /// An attempt to perform an operation on a position that does not contain a piece
    #[error("{self:?}")]
    NoPieceAtPosition,
    /// An attempt to generate king moves for a piece that is not a king
    #[error("{self:?}")]
    NotAKing,
    /// An attempt to generate regular moves for a piece that is a king
    #[error("{self:?}")]
    NotARegularPiece,
}

/// Type alias for the Result enum so that callers will not need to include the error enum in it.
pub type Result<T> = core::result::Result<T, Error>;
