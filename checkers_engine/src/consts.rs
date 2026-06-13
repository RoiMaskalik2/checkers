//! Constants that are being used by this crate

/// The amount of cells in each axis of a checkers board
pub(crate) const BOARD_SIZE: u8 = 8;

/// Used for determining a draw outcome, when 40 turns without an eat have passed, it is considered to be a draw
pub(crate) const MAX_TURNS_WITHOUT_EAT: u8 = 40;

/// Represents a positive change in position while performing a movement
pub(crate) const POSITIVE_MOVEMENT: i8 = 1;

/// Represents a negative change in position while performing a movement
pub(crate) const NEGATIVE_MOVEMENT: i8 = -1;
