//! This module contains all of the logic of checkers, without being depended on a series of moves.
//! It will receive a board, a game state, and a player that plays
//! And will perform all of the calculation independendly
//! It provides functionalities for the folliwng:
//! 1. piece move validation
//! 2. state calculation
//! 3. checkers board update after performing a move

use crate::{
    board::{Board, Move},
    consts::{BOARD_SIZE, MAX_TURNS_WITHOUT_EAT},
    err::{Error, Result},
    piece::{Piece, PieceType},
    position::{MovementDirection, Position},
    state::{Player, State, TurnState, WinState},
};

/// Represents the types of movements in a checkers game
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum MoveType {
    /// A move to an empty position in the board (does not include a game piece)
    Regular,

    /// A move that includes eating an enemy piece, includes the position where the enemy was
    Eat(Position),
}

/// Represents a move that has been validated by the logic of checkers
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct ValidMove {
    /// Validated Move
    pub(crate) movement: Move,
    /// Type of movement that was captured during move validation
    pub(crate) movement_type: MoveType,
}

impl ValidMove {
    // Generates a valid move that is a regular move
    pub(crate) fn regular(initial_position: Position, target_position: Position) -> Self {
        Self {
            movement: Move {
                initial_position,
                target_position,
            },
            movement_type: MoveType::Regular,
        }
    }
    // Generates a valid move that is an eat move
    pub(crate) fn eat(
        initial_position: Position,
        target_position: Position,
        eat_position: Position,
    ) -> Self {
        Self {
            movement: Move {
                initial_position,
                target_position,
            },
            movement_type: MoveType::Eat(eat_position),
        }
    }
}

// Generates a vector of all of the valid moves according to the state presented
// Generate only eat moves if there some eat moves
pub(crate) fn valid_moves(board: &Board, state: State, player: Player) -> Result<Vec<ValidMove>> {
    let mut moves = Vec::new();

    for (position, piece_type) in player_pieces(board, state, player)? {
        let piece = board[position].ok_or(Error::NoPieceAtPosition)?;
        for direction in piece.valid_move_directions() {
            match piece_type {
                PieceType::King => moves.extend(generate_valid_king_moves(
                    board, position, direction, player,
                )?),
                PieceType::Regular => moves.extend(generate_valid_regular_moves(
                    board, position, direction, player,
                )?),
            };
        }
    }

    let (eat_moves, regular_moves): (Vec<_>, Vec<_>) = moves
        .into_iter()
        .partition(|valid_move| matches!(valid_move.movement_type, MoveType::Eat(_)));

    Ok(if eat_moves.is_empty() {
        regular_moves
    } else {
        eat_moves
    })
}

// Generates a vector of all of the positions that the given player owns a piece in
// In MidEatTurn, only the piece at the mideat target position is returned
fn player_pieces(
    board: &Board,
    state: State,
    player: Player,
) -> Result<Vec<(Position, PieceType)>> {
    if let State::NotFinished(TurnState::MidEatTurn(position), _) = state {
        let piece = board[position].ok_or(Error::NoPieceAtPosition)?;
        return Ok(vec![(position, piece.piece_type)]);
    }

    Ok(board
        .cells()
        .into_iter()
        .filter_map(|(position, cell)| cell.map(|piece| (position, piece)))
        .filter(|(_, piece)| piece.owner == player)
        .map(|(position, piece)| (position, piece.piece_type))
        .collect())
}

// Generates a vector of all of the valid moves a king can do from the given position in the given direction
fn generate_valid_king_moves(
    board: &Board,
    position: Position,
    direction: MovementDirection,
    player: Player,
) -> Result<Vec<ValidMove>> {
    let piece = board[position].ok_or(Error::NoPieceAtPosition)?;

    if piece.piece_type != PieceType::King {
        return Err(Error::NotAKing);
    }

    if piece.owner != player {
        return Err(Error::MovingEnemyPiece);
    }

    let (mut moves, slide_end_position) = generate_king_slides(board, position, direction, player)?;

    if let Some(candidate_eat_position) = slide_end_position {
        let candidate_eat_piece = board[candidate_eat_position].ok_or(Error::NoPieceAtPosition)?;
        if let Some(eat_move) = generate_candidate_eat_move(
            board,
            position,
            candidate_eat_position,
            candidate_eat_piece,
            direction,
            player,
        ) {
            moves.push(eat_move);
        }
    }

    Ok(moves)
}

// Generates a vector valid moves a king can do given a position and a movement direction
// the vector will contain all of the moves until the king encounters a board cell that is not empty
// Also, the the position where the slide stopped will be returned
fn generate_king_slides(
    board: &Board,
    position: Position,
    direction: MovementDirection,
    player: Player,
) -> Result<(Vec<ValidMove>, Option<Position>)> {
    let piece = board[position].ok_or(Error::NoPieceAtPosition)?;

    if piece.piece_type != PieceType::King {
        return Err(Error::NotAKing);
    }

    if piece.owner != player {
        return Err(Error::MovingEnemyPiece);
    }

    let mut valid_moves = Vec::new();
    let mut current = position;

    while let Ok(next) = current.step(direction) {
        if board[next].is_some() {
            return Ok((valid_moves, Some(next)));
        }
        valid_moves.push(ValidMove::regular(position, next));
        current = next;
    }

    Ok((valid_moves, None))
}

// Generates the single valid move a regular piece can do from the given position in the given direction
fn generate_valid_regular_moves(
    board: &Board,
    position: Position,
    direction: MovementDirection,
    player: Player,
) -> Result<Option<ValidMove>> {
    let piece = board[position].ok_or(Error::NoPieceAtPosition)?;

    if piece.piece_type == PieceType::King {
        return Err(Error::NotARegularPiece);
    }

    if piece.owner != player {
        return Err(Error::MovingEnemyPiece);
    }

    let Ok(target_position) = position.step(direction) else {
        return Ok(None);
    };

    Ok(match board[target_position] {
        None => Some(ValidMove::regular(position, target_position)),
        Some(candidate_eat_piece) => generate_candidate_eat_move(
            board,
            position,
            target_position,
            candidate_eat_piece,
            direction,
            player,
        ),
    })
}

// Used when generating valid moves and there is a possible eat move
fn generate_candidate_eat_move(
    board: &Board,
    initial_position: Position,
    candidate_eat_position: Position,
    candidate_eat_piece: Piece,
    direction: MovementDirection,
    player: Player,
) -> Option<ValidMove> {
    if candidate_eat_piece.owner == player {
        return None;
    }

    let Ok(target_position) = candidate_eat_position.step(direction) else {
        return None;
    };

    if board[target_position].is_some() {
        return None;
    }

    Some(ValidMove::eat(
        initial_position,
        target_position,
        candidate_eat_position,
    ))
}

// Moves a piece accross the board given a move that has been validated
pub(crate) fn make_move(board: &mut Board, piece_move: ValidMove) -> Result<()> {
    match piece_move.movement_type {
        MoveType::Regular => board.move_cell(piece_move.movement),
        MoveType::Eat(position) => {
            board.move_cell(piece_move.movement);
            board.clear_cell(position);
        }
    }

    try_upgrade_to_king(board, piece_move.movement.target_position)?;

    Ok(())
}

// Upgrades a regular piece to a king piece if it has reached the last row in the board
fn try_upgrade_to_king(board: &mut Board, position: Position) -> Result<()> {
    let Some(piece) = &mut board[position] else {
        return Err(Error::NoPieceAtPosition);
    };

    if is_king_row(piece.owner, position) {
        piece.upgrade_to_king()?;
    }

    Ok(())
}

// Returns true if the given row is the last row for the given player
fn is_king_row(player: Player, position: Position) -> bool {
    match player {
        Player::Black => position.row == BOARD_SIZE - 1,
        Player::White => position.row == 0,
    }
}

// Get the current player from the state of the game
pub(crate) fn current_player(state: State) -> Result<Player> {
    match state {
        State::NotFinished(_, player) => Ok(player),
        State::GameOver(_) => Err(Error::FinishedGame),
    }
}

// Calculates the next game state and the valid moves available in that state after a move has been applied to the board
// The valid moves are also returned so they can be cached by the caller
pub(crate) fn next_state_and_moves(
    board: &Board,
    piece_move: ValidMove,
    current_player: Player,
    turns_since_last_eat: u8,
) -> Result<(State, Vec<ValidMove>)> {
    match piece_move.movement_type {
        MoveType::Eat(_) => next_eat_state_and_moves(board, piece_move, current_player),
        MoveType::Regular => {
            end_of_turn_state_and_moves(board, current_player, turns_since_last_eat)
        }
    }
}

// Calculates the state and valid moves after an eat move
// If the piece that just ate can eat again, returns a MidEatTurn state with those eat moves
fn next_eat_state_and_moves(
    board: &Board,
    piece_move: ValidMove,
    current_player: Player,
) -> Result<(State, Vec<ValidMove>)> {
    let mid_eat_state = State::NotFinished(
        TurnState::MidEatTurn(piece_move.movement.target_position),
        current_player,
    );

    let next_eats = valid_moves(board, mid_eat_state, current_player)?;
    if next_eats
        .iter()
        .any(|valid_move| matches!(valid_move.movement_type, MoveType::Eat(_)))
    {
        return Ok((mid_eat_state, next_eats));
    }

    end_of_turn_state_and_moves(board, current_player, 0)
}

// Calculates the state and valid moves at the end of a turn, switching to the enemy
fn end_of_turn_state_and_moves(
    board: &Board,
    current_player: Player,
    turns_since_last_eat: u8,
) -> Result<(State, Vec<ValidMove>)> {
    if turns_since_last_eat >= MAX_TURNS_WITHOUT_EAT {
        return Ok((State::GameOver(WinState::Draw), Vec::new()));
    }

    let enemy = !current_player;
    let enemy_state = State::NotFinished(TurnState::RegularTurn, enemy);

    let enemy_moves = valid_moves(board, enemy_state, enemy)?;
    if !enemy_moves.is_empty() {
        return Ok((enemy_state, enemy_moves));
    }

    game_over_state_and_moves(board, current_player)
}

// Calculates the game over state when the enemy has no available moves
// Returns a win for the current player if they still have moves, else a draw
fn game_over_state_and_moves(
    board: &Board,
    current_player: Player,
) -> Result<(State, Vec<ValidMove>)> {
    let current_state = State::NotFinished(TurnState::RegularTurn, current_player);
    let current_moves = valid_moves(board, current_state, current_player)?;

    let state = if current_moves.is_empty() {
        State::GameOver(WinState::Draw)
    } else {
        State::GameOver(WinState::Win(current_player))
    };

    Ok((state, Vec::new()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_unordered::assert_eq_unordered;

    fn board_with_black(position: Position, piece_type: PieceType) -> Board {
        let mut board = Board::empty();
        board[position] = Some(Piece {
            owner: Player::Black,
            piece_type,
        });

        board
    }

    fn board_with_white(position: Position, piece_type: PieceType) -> Board {
        let mut board = Board::empty();
        board[position] = Some(Piece {
            owner: Player::White,
            piece_type,
        });

        board
    }

    fn place_black(board: &mut Board, position: Position, piece_type: PieceType) {
        board[position] = Some(Piece {
            owner: Player::Black,
            piece_type,
        });
    }

    fn place_white(board: &mut Board, position: Position, piece_type: PieceType) {
        board[position] = Some(Piece {
            owner: Player::White,
            piece_type,
        });
    }

    fn board_with_both(black_position: Position, white_position: Position) -> Board {
        let mut board = Board::empty();
        board[black_position] = Some(Piece {
            owner: Player::Black,
            piece_type: PieceType::Regular,
        });
        board[white_position] = Some(Piece {
            owner: Player::White,
            piece_type: PieceType::Regular,
        });

        board
    }

    #[test]
    fn current_player_from_not_finished_state() -> Result<()> {
        let state = State::NotFinished(TurnState::RegularTurn, Player::Black);
        assert_eq!(current_player(state)?, Player::Black);

        let state = State::NotFinished(TurnState::RegularTurn, Player::White);
        assert_eq!(current_player(state)?, Player::White);

        Ok(())
    }

    #[test]
    fn current_player_from_game_over_returns_error() -> Result<()> {
        let state = State::GameOver(WinState::Draw);
        assert!(matches!(current_player(state), Err(Error::FinishedGame)));

        let state = State::GameOver(WinState::Win(Player::Black));
        assert!(matches!(current_player(state), Err(Error::FinishedGame)));

        let state = State::GameOver(WinState::Win(Player::White));
        assert!(matches!(current_player(state), Err(Error::FinishedGame)));

        Ok(())
    }

    #[test]
    fn valid_moves_black_piece_moves_only_up() -> Result<()> {
        let board = board_with_black(Position { row: 3, column: 3 }, PieceType::Regular);
        let state = State::NotFinished(TurnState::RegularTurn, Player::Black);
        let moves = valid_moves(&board, state, Player::Black)?;
        let target_positions: Vec<Position> = moves
            .iter()
            .map(|valid_move| valid_move.movement.target_position)
            .collect();

        assert_eq_unordered!(
            target_positions,
            vec![
                Position { row: 4, column: 4 },
                Position { row: 4, column: 2 }
            ]
        );

        Ok(())
    }

    #[test]
    fn valid_moves_white_piece_moves_only_down() -> Result<()> {
        let board = board_with_white(Position { row: 3, column: 3 }, PieceType::Regular);
        let state = State::NotFinished(TurnState::RegularTurn, Player::White);
        let moves = valid_moves(&board, state, Player::White)?;

        let target_positions: Vec<Position> = moves
            .iter()
            .map(|valid_move| valid_move.movement.target_position)
            .collect();

        assert_eq_unordered!(
            target_positions,
            vec![
                Position { row: 2, column: 4 },
                Position { row: 2, column: 2 }
            ]
        );

        Ok(())
    }

    #[test]
    fn valid_moves_king_moves_in_all_directions() -> Result<()> {
        let board = board_with_black(Position { row: 3, column: 3 }, PieceType::King);
        let state = State::NotFinished(TurnState::RegularTurn, Player::Black);
        let moves = valid_moves(&board, state, Player::Black)?;

        let target_positions: Vec<Position> = moves
            .iter()
            .map(|valid_move| valid_move.movement.target_position)
            .collect();

        assert!(target_positions.contains(&Position { row: 2, column: 4 }));
        assert!(target_positions.contains(&Position { row: 2, column: 2 }));
        assert!(target_positions.contains(&Position { row: 4, column: 4 }));
        assert!(target_positions.contains(&Position { row: 4, column: 2 }));

        Ok(())
    }

    #[test]
    fn valid_moves_king_slides_multiple_cells() -> Result<()> {
        let board = board_with_black(Position { row: 0, column: 0 }, PieceType::King);
        let state = State::NotFinished(TurnState::RegularTurn, Player::Black);

        let moves = valid_moves(&board, state, Player::Black)?;
        assert_eq!(moves.len(), 7);

        Ok(())
    }

    #[test]
    fn valid_moves_must_eat() -> Result<()> {
        let board = board_with_both(
            Position { row: 3, column: 3 },
            Position { row: 4, column: 4 },
        );
        let state = State::NotFinished(TurnState::RegularTurn, Player::Black);

        let moves = valid_moves(&board, state, Player::Black)?;
        assert_eq!(moves.len(), 1);
        assert!(matches!(
            moves.first().unwrap().movement_type,
            MoveType::Eat(_)
        ));

        Ok(())
    }

    #[test]
    fn valid_moves_empty_when_player_has_no_pieces() -> Result<()> {
        let board = Board::empty();

        let state = State::NotFinished(TurnState::RegularTurn, Player::Black);
        let moves = valid_moves(&board, state, Player::Black)?;
        assert!(moves.is_empty());

        let state = State::NotFinished(TurnState::RegularTurn, Player::White);
        let moves = valid_moves(&board, state, Player::White)?;
        assert!(moves.is_empty());

        Ok(())
    }

    #[test]
    fn valid_moves_no_eat_when_target_has_owner() -> Result<()> {
        let mut board = board_with_both(
            Position { row: 3, column: 3 },
            Position { row: 4, column: 4 },
        );
        place_white(
            &mut board,
            Position { row: 5, column: 5 },
            PieceType::Regular,
        );
        let state = State::NotFinished(TurnState::RegularTurn, Player::Black);

        let moves = valid_moves(&board, state, Player::Black)?;
        assert_eq!(moves.len(), 1);
        assert!(matches!(
            moves.first().unwrap().movement_type,
            MoveType::Regular
        ));

        Ok(())
    }

    #[test]
    fn valid_moves_no_eat_when_target_is_out_of_board() -> Result<()> {
        let board = board_with_both(
            Position { row: 6, column: 6 },
            Position { row: 7, column: 7 },
        );
        let state = State::NotFinished(TurnState::RegularTurn, Player::Black);

        let moves = valid_moves(&board, state, Player::Black)?;
        assert_eq!(moves.len(), 1);
        assert!(matches!(
            moves.first().unwrap().movement_type,
            MoveType::Regular
        ));

        Ok(())
    }

    #[test]
    fn valid_moves_mid_eat_only_form_eating_position() -> Result<()> {
        let eating_position = Position { row: 4, column: 4 };
        let mut board = board_with_black(eating_position, PieceType::Regular);
        place_black(
            &mut board,
            Position { row: 2, column: 2 },
            PieceType::Regular,
        );
        let state = State::NotFinished(TurnState::MidEatTurn(eating_position), Player::Black);

        let moves = valid_moves(&board, state, Player::Black)?;
        assert_eq!(moves.len(), 2);
        assert!(
            moves
                .iter()
                .all(|valid_move| valid_move.movement.initial_position == eating_position)
        );

        Ok(())
    }

    #[test]
    fn make_eat_move_removes_eated_piece() -> Result<()> {
        let mut board = board_with_both(
            Position { row: 3, column: 3 },
            Position { row: 4, column: 4 },
        );

        make_move(
            &mut board,
            ValidMove::eat(
                Position { row: 3, column: 3 },
                Position { row: 5, column: 5 },
                Position { row: 4, column: 4 },
            ),
        )?;
        assert!(board[Position { row: 4, column: 4 }].is_none());

        Ok(())
    }

    #[test]
    fn make_move_upgrades_black_piece_at_last_row() -> Result<()> {
        let source = Position { row: 6, column: 2 };
        let target = Position { row: 7, column: 3 };
        let mut board = board_with_black(source, PieceType::Regular);

        make_move(&mut board, ValidMove::regular(source, target))?;
        assert_eq!(board[target].unwrap().piece_type, PieceType::King);

        Ok(())
    }

    #[test]
    fn make_move_upgrades_white_piece_at_first_row() -> Result<()> {
        let source = Position { row: 1, column: 2 };
        let target = Position { row: 0, column: 1 };
        let mut board = board_with_white(source, PieceType::Regular);

        make_move(&mut board, ValidMove::regular(source, target))?;
        assert_eq!(board[target].unwrap().piece_type, PieceType::King);

        Ok(())
    }

    #[test]
    fn next_state_switches_to_enemy_after_regular_move() -> Result<()> {
        let board = Board::new();
        let piece_move = ValidMove::regular(
            Position { row: 2, column: 2 },
            Position { row: 3, column: 3 },
        );

        let (state, _) = next_state_and_moves(&board, piece_move, Player::Black, 0)?;
        assert!(matches!(
            state,
            State::NotFinished(TurnState::RegularTurn, Player::White)
        ));

        Ok(())
    }

    #[test]
    fn next_state_is_draw_after_max_turns_without_eat() -> Result<()> {
        let board = Board::new();
        let piece_move = ValidMove::regular(
            Position { row: 2, column: 2 },
            Position { row: 3, column: 3 },
        );

        let (state, _) =
            next_state_and_moves(&board, piece_move, Player::Black, MAX_TURNS_WITHOUT_EAT)?;
        assert!(matches!(state, State::GameOver(WinState::Draw)));

        Ok(())
    }

    #[test]
    fn next_state_is_mid_eat_when_there_is_double_eat() -> Result<()> {
        let target_position = Position { row: 4, column: 4 };
        let board = board_with_both(target_position, Position { row: 5, column: 5 });
        let piece_move = ValidMove::eat(
            Position { row: 2, column: 2 },
            target_position,
            Position { row: 3, column: 3 },
        );

        let (state, moves) = next_state_and_moves(&board, piece_move, Player::Black, 0)?;
        assert!(matches!(
            state,
            State::NotFinished(
                TurnState::MidEatTurn(Position { row: 4, column: 4 }),
                Player::Black
            )
        ));
        assert_eq!(moves.len(), 1);
        assert!(matches!(
            moves.first().unwrap().movement_type,
            MoveType::Eat(_)
        ));

        Ok(())
    }

    #[test]
    fn next_state_switches_to_enemy_after_eat_with_no_doube_eat() -> Result<()> {
        let target_position = Position { row: 4, column: 4 };
        let board = board_with_both(target_position, Position { row: 7, column: 1 });
        let piece_move = ValidMove::eat(
            Position { row: 2, column: 2 },
            target_position,
            Position { row: 3, column: 3 },
        );

        let (state, _) = next_state_and_moves(&board, piece_move, Player::Black, 0)?;
        assert!(matches!(
            state,
            State::NotFinished(TurnState::RegularTurn, Player::White)
        ));

        Ok(())
    }

    #[test]
    fn next_state_is_win_when_enemy_has_no_pieces() -> Result<()> {
        let target_position = Position { row: 4, column: 4 };
        let board = board_with_black(target_position, PieceType::Regular);
        let piece_move = ValidMove::eat(
            Position { row: 2, column: 2 },
            target_position,
            Position { row: 3, column: 3 },
        );

        let (state, _) = next_state_and_moves(&board, piece_move, Player::Black, 0)?;
        assert!(matches!(
            state,
            State::GameOver(WinState::Win(Player::Black))
        ));

        Ok(())
    }
}
