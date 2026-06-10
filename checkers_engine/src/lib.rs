//! Checkers Engine
//!
//! Exports the building blocks for interacting with a checkers engine

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// TODO: Figure how which of them should be public and which should not
mod board;
mod consts;
mod engine;
mod err;
mod piece;
mod position;
mod state;

/// Figure out what should be public - for example: the Board struct should not be a public API
pub use board::{Board, Move};
pub use err::{Error, Result};
pub use piece::{Piece, PieceType};
pub use position::{MovementDirection, Position};
pub use state::{Player, State, TurnState, WinState};
