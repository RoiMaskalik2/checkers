//! This module represents the different states a checkers game can be in.

use crate::Position;

/// Represents the type of players that play a checkers game
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    /// Has black checkers pieces
    Black,

    /// Has white checkers pieces
    White,
}

/// Represents the state of a checkers game
pub enum State {
    /// The game has ended
    GameOver(WinState),

    /// The game is still ongoing
    NotFinished(TurnState, Player),
}

/// Represents the state of the game when it has finished
pub enum WinState {
    /// Some player has won the game.
    Win(Player),

    /// The game ended on a tie.
    Draw,
}

/// Represents the state of the game when it is ongoing
pub enum TurnState {
    /// The game is being played and it is a regular turn
    RegularTurn,

    /// The game is being played and it is another turn after burn
    MidEatTurn(Position),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Result;

    #[test]
    fn not_player() -> Result<()> {
        let player = !Player::White;
        assert_eq!(player, Player::Black);

        let player = !Player::Black;
        assert_eq!(player, Player::White);

        Ok(())
    }
}
