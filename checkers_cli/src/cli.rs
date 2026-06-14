//! This module implements an interactive API to get checkers moves through CLI commands.

use checkers_engine::{Move, Position};
use clap::Parser;
/// This struct represents the arguments from the cli of a checkers move
#[derive(Debug, Parser)]
#[command(no_binary_name = true)]
pub struct MoveArguments {
    /// Initial position row
    initial_row: u8,
    /// Initial position column
    initial_column: u8,
    /// Target position row
    target_row: u8,
    /// Target position column
    target_column: u8,
}

impl TryFrom<MoveArguments> for Move {
    type Error = checkers_engine::Error;
    fn try_from(arguments: MoveArguments) -> Result<Self, Self::Error> {
        let initial_position = Position {
            row: arguments.initial_row,
            column: arguments.initial_column,
        };

        let target_position = Position {
            row: arguments.target_row,
            column: arguments.target_column,
        };

        Position::validate(initial_position)?;
        Position::validate(target_position)?;

        Ok(Self {
            initial_position,
            target_position,
        })
    }
}
