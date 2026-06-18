use checkers_cli::{MoveArguments, Result, user_input::read_checkers_move};
use checkers_engine::{
    CheckersEngine, Move, Player,
    State::{self, GameOver, NotFinished},
};
use clap::CommandFactory;

fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    let mut engine = CheckersEngine::new();
    let starting_player = Player::White;

    MoveArguments::command().print_help()?;
    play_checkers(&mut engine, starting_player);

    Ok(())
}

// Takes input from the user until the game ends
// Prints the board the the state of the game
// Prints additional information if the input was invalid
fn play_checkers(engine: &mut CheckersEngine, starting_player: Player) {
    let mut current_player = starting_player;
    loop {
        println!("{engine}");

        match play_checkers_turn(engine, current_player) {
            Ok(GameOver(_)) => break,
            Ok(NotFinished(_, player)) => current_player = player,
            Err(error) => println!("{error}"),
        }
    }
}

// Takes an input from the user and converts it into a checkers move
// Returns error if the move was invalid
fn play_checkers_turn(engine: &mut CheckersEngine, player: Player) -> Result<State> {
    let move_arguments = read_checkers_move()?;
    let piece_move = Move::try_from(move_arguments)?;

    Ok(engine.play_turn(piece_move, player)?)
}
