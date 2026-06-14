//! Errors that can occur in this crate, grouped by the module they came from.

use std::io;

/// Represents an error that can occur while using the checkers cli crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // ---- user_input --------------------------------------------
    /// User provided an empty input.
    #[error("{self:?}")]
    EmptyString,

    /// Error occurred during input reading.
    #[error("{self:?}")]
    Io(#[from] io::Error),

    /// Error occurred during converting an input string to a checkers move command.
    #[error("{self:?}")]
    ParseCommand(#[from] shell_words::ParseError),

    /// The move arguments where not passed in the correct format
    #[error("{self:?}")]
    InvalidCliChoice(#[from] clap::Error),

    // ---- cli --------------------------------------------
    /// error occured during playing the checkers engine
    #[error("{0}")]
    EngineError(#[from] checkers_engine::Error),
}

/// Type alias for the Result enum so that callers will not need to include the error enum in it.
pub type Result<T> = core::result::Result<T, Error>;
