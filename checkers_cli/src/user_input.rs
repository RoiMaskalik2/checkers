//! User input handling module.
//!
//! includes functions for  user input from the command line.
use crate::{Error, MoveArguments, Result};
use clap::Parser;
use std::io;

/// Reads a string from the standard input, trims it, and checks that the input is not empty.
pub fn input_string(display_message: &str) -> Result<String> {
    println!("{}", display_message);

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    user_input.truncate(user_input.trim_end().len());

    (!user_input.is_empty())
        .then_some(())
        .ok_or(Error::EmptyString)?;

    Ok(user_input)
}

pub fn read_checkers_move() -> Result<MoveArguments> {
    let user_command = input_string("Command: ")?;

    let user_choice = MoveArguments::try_parse_from(shell_words::split(&user_command)?.iter())?;

    Ok(user_choice)
}
