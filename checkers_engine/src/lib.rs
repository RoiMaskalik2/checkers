//! Checkers Engine
//!
//! Exports the building blocks for interacting with a checkers engine

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod board;
mod consts;
mod engine;
mod err;
mod logic;
mod piece;
mod position;
mod state;

pub use board::{Cell, Move};
pub use engine::CheckersEngine;
pub use err::{Error, Result};
pub use piece::{Piece, PieceType};
pub use position::Position;
pub use state::{Player, State, TurnState, WinState};
