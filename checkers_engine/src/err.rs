//! Errors that can occur in this crate, grouped by the module they came from.

/// Represents an error that can occur while using the checkers engine crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // ---- position module --------------------------------------------
    /// Position is not in the valid board range (between 0-7 in both AXES of the board)
    #[error("{self:?}")]
    InvalidPosition,
}

/// Type alias for the Result enum so that callers will not need to include the error enum in it.
pub type Result<T> = core::result::Result<T, Error>;
