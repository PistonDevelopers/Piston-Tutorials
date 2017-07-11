//! Game board logic.

/// Size of game board.
const SIZE: usize = 9;

/// Stores game board information.
pub struct Gameboard {
    /// Stores the content of the cells.
    /// `0` is an empty cell.
    pub cells: [[u8; SIZE]; SIZE],
}

impl Gameboard {
    /// Creates a new game board.
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[0; SIZE]; SIZE],
        }
    }
}
