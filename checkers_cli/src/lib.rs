//! Checkers Cli
//!
//! Exports the building blocks for interactively playing checkers through a cli

mod cli;
mod err;
pub mod user_input;

pub use cli::MoveArguments;
pub use err::{Error, Result};
