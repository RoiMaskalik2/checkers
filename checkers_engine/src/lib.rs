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

pub use err::{Error, Result};
pub use position::{MovementDirection, Position};
